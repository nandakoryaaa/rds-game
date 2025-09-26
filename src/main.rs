#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

pub mod static_drawable;
pub mod renderer;
pub mod factory;
pub mod pantry;
pub mod controller;
pub mod input;
pub mod behaviour;
pub mod game;
pub mod collider;
pub mod xrand;
pub mod zlib;
pub mod png;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{ Texture, BlendMode, TextureAccess };
//use sdl2::keyboard::Keycode;

use std::{ thread, time };

//use static_drawable::*;
use controller::*;
use input::*;
use renderer::*;
use factory::*;
use pantry::*;
use behaviour::*;
use game::*;
//use collider::*;
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

pub fn main()
{
	let img = png::read_file("rds_atlas.png");

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

	canvas.set_blend_mode(BlendMode::Blend);

	let texture_creator = canvas.texture_creator();

	let mut texture = texture_creator.create_texture(
		PixelFormatEnum::ABGR8888,
		TextureAccess::Static,
		img.width,
		img.height
	).unwrap();

	let _ = texture.update(None, &img.data[0..], img.stride);
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

	let mut evt_pump = sdl.event_pump().unwrap();
	let timer = sdl.timer().unwrap();
	let mut running = true;
	let mut next_tick: i32 = timer.ticks() as i32 + FPS_DELAY;

	let mut controller_title = ControllerTitle::new();
	let mut controller_game = ControllerGame::new(MAX_OBJ_CNT);

	let mut controller: &mut dyn Controller = &mut controller_title;
	let mut input = InputBuilder::any_key();

	controller.begin(&mut ctx);

	while running {
		renderer.clear();
		ctx.stage.draw(&mut renderer);
		renderer.present();

		for evt in evt_pump.poll_iter() {
			match evt {
				Event::Quit { .. } => {
					running = false;
				},
				Event::KeyDown { .. } | Event::KeyUp { .. } => {
					input.set_event(&evt);
				},
				_ => ()
			}
		}

		let evt = controller.run(&mut ctx, &input);
		if evt == ControllerEvent::EndTitle {
			controller.end(&mut ctx);
			controller = &mut controller_game;
			input = InputBuilder::game();
			controller.begin(&mut ctx);
		} else if evt == ControllerEvent::EndGame {
			controller.end(&mut ctx);
			running = false;
		}

		let mut diff: i32 = next_tick - timer.ticks() as i32;
		while diff > 0 {
			thread::sleep(time::Duration::from_millis(diff as u64));
			diff = next_tick - timer.ticks() as i32;
		}
		next_tick += FPS_DELAY;
	}

	controller.end(&mut ctx);
}
