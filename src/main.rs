#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

pub mod static_drawable;
pub mod renderer;
pub mod factory;
pub mod pantry;
pub mod behaviour;
pub mod game;
pub mod collider;
pub mod xrand;
pub mod zlib;
pub mod png;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{ Texture, BlendMode, TextureAccess };
use sdl2::keyboard::Keycode;

use std::{ thread, time };

use static_drawable::*;
use renderer::*;
use factory::*;
use pantry::*;
use behaviour::*;
use game::*;
use collider::*;
use xrand::XRand;

const MAX_OBJ_CNT: usize = 128;
const FPS_DELAY: i32 = 33;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct Storage {
	pub pantry_bhvd_move: Pantry<BhvDataMove>,
	pub pantry_bhvd_tm: Pantry<BhvDataTimedMotion>,
	pub pantry_bhvd_gun: Pantry<BhvDataGun>
}

impl Storage {
	pub fn create(capacity: usize) -> Self {
		Storage {
			pantry_bhvd_move: Pantry::create(capacity),
			pantry_bhvd_tm: Pantry::create(capacity),
			pantry_bhvd_gun: Pantry::create(1)
		}
	}
}

pub struct Context {
	pub stage: Stage,
	pub storage: Storage,
	pub gmo_factory: GmoFactory,
	pub sto_factory: StoFactory,
	pub vec_gmo_new: Vec<GmoNew>,
	pub rand: XRand
}

fn process_game_objects(
	pantry_gmo: &mut Pantry<GameObject>, ctx: &mut Context
) {
	if pantry_gmo.len() == 0 {
		return;
	}

	let mut index = pantry_gmo.first_index();
	loop {
		// запомнить состояние последнего индекса,
		// т.к. после pantry_gmo.free() он может измениться
		let is_last = pantry_gmo.is_last_index(index);
		let gmo = pantry_gmo.get_mut(index);
		let status = gmo.bhv.update(ctx, gmo);
		if status == BhvStatus::END {
			gmo.free(ctx);
			pantry_gmo.free(index);
		} else {
			let sto = ctx.stage.get_mut(gmo.sto_index);
			sto.x = gmo.data.x;
			sto.y = gmo.data.y;
		}
		if is_last {
			break;
		}
		index = pantry_gmo.next_index(index);
	}
}

fn print_game_objects(pantry_gmo: &Pantry<GameObject>)
{
	println!("gmo list:");
	if pantry_gmo.used_cnt == 0 {
		return;
	}

	let mut index = pantry_gmo.first_index();
	let last_index = pantry_gmo.last_index();
	loop {
		let gmo = pantry_gmo.get(index);
		println!("{} type: {} x: {} y: {} w: {} h: {}", index, gmo.gmo_type as u8, gmo.data.x, gmo.data.y, gmo.data.w, gmo.data.h);
		if index == last_index {
			break;
		}
		index = pantry_gmo.next_index(index);
	}
}

pub fn main()
{
	let sdl = sdl2::init().unwrap();
	let vss: sdl2::VideoSubsystem = sdl.video().unwrap();

	let wb = sdl2::video::WindowBuilder::new(
		& vss,
		"RDS",
		WINDOW_WIDTH,
		WINDOW_HEIGHT
	);

	let img = png::read_file("rds_atlas.png");

	let window: sdl2::video::Window = wb.build().unwrap();
	let cb = sdl2::render::CanvasBuilder::new(window);
	let mut canvas = cb.build().unwrap();

	canvas.set_blend_mode(BlendMode::Blend);

	let texture_creator = canvas.texture_creator();

	let mut texture = texture_creator.create_texture(
		PixelFormatEnum::ABGR8888,
		TextureAccess::Static,
		img.width,
		img.height
	).unwrap();

	texture.update(None, &img.data[0..], img.stride);
	texture.set_blend_mode(BlendMode::Blend);

	let mut renderer = Renderer {
		canvas: &mut canvas,
		texture_list: Vec::<Texture>::with_capacity(2)
	};

	renderer.texture_list.push(texture);

	let mut ctx = Context {
		stage: Stage {
			w: WINDOW_WIDTH,
			h: WINDOW_HEIGHT,
			pantry_sto: Pantry::create(MAX_OBJ_CNT),
		},
		gmo_factory: GmoFactory {},
		sto_factory: StoFactory {},
		storage: Storage::create(MAX_OBJ_CNT),
		vec_gmo_new: Vec::with_capacity(MAX_OBJ_CNT),
		rand: XRand::new()
	};

	let mut pantry_gmo: Pantry<GameObject> = Pantry::create(MAX_OBJ_CNT);
	let mut vec_collide: Vec<CollidePair> = Vec::with_capacity(MAX_OBJ_CNT);

	let gmo_factory = ctx.gmo_factory;

	{
		let mut gmo_gun = gmo_factory.spawn_gun(
			&mut ctx, 400-16, 600-37,
			BhvDataGun { wave_type: GmoType::CARRIER, cnt: 10, delay: 30 }
		);
		gmo_gun.sto_index = ctx.stage.add_child(
			ctx.sto_factory.spawn_gun(gmo_gun.data.x, gmo_gun.data.y)
		);
		pantry_gmo.alloc(gmo_gun);
	}

	let shaft_index = ctx.stage.add_child(StageObject { x: 400, y: 600-32, angle: 90, drawable: &DR_SHAFT });

	let mut evt_pump = sdl.event_pump().unwrap();
	let timer = sdl.timer().unwrap();
	let mut running = true;
	let mut next_tick: i32 = timer.ticks() as i32 + FPS_DELAY;

	let collider = Collider {};
	let solver = Solver {};

	while running {

		renderer.clear();
		ctx.stage.draw(&mut renderer);
		renderer.present();

		for evt in evt_pump.poll_iter() {
			match evt {
				Event::Quit { .. } => {
					running = false;
				},
				Event::KeyDown { keycode: Some(k), .. } => {
					if k == Keycode::I {
						print_game_objects(&pantry_gmo);
					} else if k == Keycode::Left {
						let sto = ctx.stage.get_mut(shaft_index);
						sto.angle += 3;
						if sto.angle > 180 {
							sto.angle = 180;
						}
					} else if k == Keycode::Right {
						let sto = ctx.stage.get_mut(shaft_index);
						sto.angle -= 3;
						if sto.angle < 0 {
							sto.angle = 0;
						}
					} else {
						let sto = ctx.stage.get(shaft_index);
						let theta: f32 = (sto.angle as f32) * 3.1415926 / 180.0;
						let cos = theta.cos();
						let sin = theta.sin();
						let x = sto.x + (20.0 * cos).round() as i32;
						let y = sto.y - (20.0 * sin).round() as i32;
						let gmo_factory = ctx.gmo_factory;
						let mut gmo_shot = gmo_factory.spawn_shot(
							&mut ctx, x, y,
							BhvDataMove {
								dx: (5.0 * cos).round() as i32,
								dy: -(5.0 * sin).round() as i32
							}
						);
						let sto_shot = ctx.sto_factory.spawn_shot(gmo_shot.data.x, gmo_shot.data.y);
						gmo_shot.sto_index = ctx.stage.add_child(sto_shot);
						pantry_gmo.alloc(gmo_shot);
					}
				},
				_ => ()
			}
		}
// uncomment for autoplay
/*
					let r = ctx.rand.randint(0, 1000);
					if r < 450 {
						let sto = ctx.stage.get_mut(shaft_index);
						sto.angle += 3;
						if sto.angle > 180 {
							sto.angle = 180;
						}
					} else if r > 550 {
						let sto = ctx.stage.get_mut(shaft_index);
						sto.angle -= 3;
						if sto.angle < 0 {
							sto.angle = 0;
						}
					} else if ctx.rand.randint(0, 1000) > 750 {
						let sto = ctx.stage.get(shaft_index);
						let theta: f32 = (sto.angle as f32) * 3.1415926 / 180.0;
						let cos = theta.cos();
						let sin = theta.sin();
						let x = sto.x + (20.0 * cos).round() as i32;
						let y = sto.y - (20.0 * sin).round() as i32;
						let gmo_factory = ctx.gmo_factory;
						let mut gmo_shot = gmo_factory.spawn_shot(
							&mut ctx, x, y,
							BhvDataMove {
								dx: (5.0 * cos).round() as i32,
								dy: -(5.0 * sin).round() as i32
							}
						);
						let sto_shot = ctx.sto_factory.spawn_shot(gmo_shot.data.x, gmo_shot.data.y);
						gmo_shot.sto_index = ctx.stage.add_child(sto_shot);
						pantry_gmo.alloc(gmo_shot);
					}
*/
		process_game_objects(&mut pantry_gmo, &mut ctx);

		collider.check(
			PlainRect { x: 0, y: 0, w: ctx.stage.w, h: ctx.stage.h },
			&mut pantry_gmo,
			&mut vec_collide
		);

		if vec_collide.len() > 0 {
			let sevt = solver.solve(&mut pantry_gmo, &mut vec_collide, &mut ctx);
			let score = 10 * sevt.shot_carriers + 5 * sevt.shot_chutes;
			if score > 0 {
				//println!("score: {}", score);
			}
			vec_collide.clear();
		}

		while ctx.vec_gmo_new.len() > 0 {
			let mut new = ctx.vec_gmo_new.pop().unwrap();
			new.gmo.sto_index = ctx.stage.add_child(new.sto);
			pantry_gmo.alloc(new.gmo);
		}

		let mut diff: i32 = next_tick - timer.ticks() as i32;
		while diff > 0 {
			thread::sleep(time::Duration::from_millis(diff as u64));
			diff = next_tick - timer.ticks() as i32;
		}
		next_tick += FPS_DELAY;

	}
}
