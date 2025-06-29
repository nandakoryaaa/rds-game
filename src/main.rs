#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

pub mod renderer;
pub mod factory;
pub mod pantry;
pub mod behaviour;
pub mod game;
pub mod collider;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::{thread, time};

use crate::renderer::*;
use crate::factory::*;
use crate::pantry::*;
use crate::behaviour::*;
use crate::game::*;
use crate::collider::*;

const FPS_DELAY: i32 = 33;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub static DR_SHOT: DrawableRect = DrawableRect {
	w: 3, h: 3, color: Color::RGB(255, 255, 255)
};
pub static DR_CARRIER: DrawableRect = DrawableRect {
	w: 40, h: 20, color: Color::RGB(255, 255, 0)
};
pub static DR_BOMBER: DrawableRect = DrawableRect {
	w: 30, h: 20, color: Color::RGB(0, 0, 255)
};
pub static DR_CHUTE: DrawableRect = DrawableRect {
	w: 15, h: 20, color: Color::RGB(0, 255, 0)
};
pub static DR_BOMB: DrawableRect = DrawableRect {
	w: 10, h: 10, color: Color::RGB(255, 0, 0)
};

pub struct Storage {
	pub pantry_bhvd_move: Pantry<BhvDataMove>,
	pub pantry_bhvd_carrier: Pantry<BhvDataCarrier>,
}

impl Storage {
	pub fn create(capacity: usize) -> Self {
		Storage {
			pantry_bhvd_move: Pantry::create(capacity),
			pantry_bhvd_carrier: Pantry::create(capacity),
		}
	}
}

pub struct Context<'a> {
	pub stage: Stage<'a>,
	pub storage: Storage,
	pub factory: GmoFactory,
	pub vec_gmo_new: Vec<GameObject>
}

fn process_game_objects(
	pantry_gmo: &mut Pantry<GameObject>, ctx: &mut Context
) {
	if pantry_gmo.used_cnt == 0 {
		return;
	}

	let mut index = pantry_gmo.first_index();
	let last_index = pantry_gmo.last_index();
	loop {
		let gmo = pantry_gmo.get(index);
		gmo.bhv.update(&mut gmo.data, ctx, gmo.bhvd_index);
		let x = gmo.data.x;
		let y = gmo.data.y;
		if x < 0 || y < 0 || x as u32 >= ctx.stage.w || y as u32 >= ctx.stage.h {
			gmo.free(ctx);
			pantry_gmo.free(index);
		} else {
			let sto = ctx.stage.get(gmo.sto_index);
			sto.x = x;
			sto.y = y;
		}
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

	let window: sdl2::video::Window = wb.build().unwrap();
	let cb = sdl2::render::CanvasBuilder::new(window);
	let mut canvas = cb.build().unwrap();
	let mut renderer = Renderer { canvas: &mut canvas };

	let mut ctx = Context {
		stage: Stage {
			w: WINDOW_WIDTH,
			h: WINDOW_HEIGHT,
			pantry_sto: Pantry::create(128),
		},
		factory: GmoFactory {},
		storage: Storage::create(128),
		vec_gmo_new: Vec::with_capacity(128)
	};

	let mut pantry_gmo: Pantry<GameObject> = Pantry::create(128);
	let mut vec_collide: Vec<CollidePair> = Vec::with_capacity(128);

	let factory = ctx.factory;

	pantry_gmo.alloc(
		factory.spawn_carrier(&mut ctx,
			GmoData { x: 700, y: 20, w:40, h:20 },
			BhvDataCarrier { dx: -1, interval: 30, cnt: 0 }
		)
	);
	pantry_gmo.alloc(
		factory.spawn_carrier(&mut ctx,
			GmoData { x: 0, y: 40, w:40, h:20 },
			BhvDataCarrier { dx: 1, interval: 30, cnt: 0 }
		)
	);

	let mut evt_pump = sdl.event_pump().unwrap();
	let timer = sdl.timer().unwrap();
	let mut running = true;
	let mut next_tick: i32 = timer.ticks() as i32 + FPS_DELAY;

	let collider = Collider {};
	let solver = Solver {};

	while running {
		ctx.stage.draw(&mut renderer);
		for evt in evt_pump.poll_iter() {
			match evt {
				Event::Quit { .. } => {
					running = false;
				},
				Event::KeyDown { keycode: Some(k), .. } => {
					let factory = ctx.factory;
					pantry_gmo.alloc(
						factory.spawn_shot(
							&mut ctx,
							GmoData { x: 400, y: 599, w:3, h:3 },
							BhvDataMove { dx: 0, dy: -5 }
						)
					);
				},
				_ => ()
			}
		}

		let mut diff: i32 = next_tick - timer.ticks() as i32;
		while diff > 0 {
			thread::sleep(time::Duration::from_millis(diff as u64));
			diff = next_tick - timer.ticks() as i32;
		}

		process_game_objects(&mut pantry_gmo, &mut ctx);
		collider.check(&mut pantry_gmo, &mut vec_collide);
		if vec_collide.len() > 0 {
			solver.solve(&mut pantry_gmo, &mut vec_collide, &mut ctx);
			vec_collide.clear();
		}
		for i in 0..ctx.vec_gmo_new.len() {
			pantry_gmo.alloc(ctx.vec_gmo_new.pop().unwrap());
		}

		next_tick += FPS_DELAY;
	}
}
