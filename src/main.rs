#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

pub mod renderer;
pub mod factory;
pub mod pantry;
pub mod behaviour;
pub mod game;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::{thread, time};

use crate::renderer::*;
use crate::factory::*;
use crate::pantry::*;
use crate::behaviour::*;
use crate::game::*;


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

pub struct Shot {}
impl Shot {
	pub fn new(ctx: &mut Context, gmo_data:GmoData, bhv_data: BhvDataMove) -> GameObject {
		GameObject {
			sto_index: ctx.stage.add_child(
				StageObject {
					x: gmo_data.x, y: gmo_data.y,
					drawable: &DR_SHOT
				}
			),
			data: gmo_data,
			bhv: &BehaviourMove {},
			bhvd_index: ctx.storage.pantry_bhvd_move.alloc(bhv_data)
		}
	}
}

pub struct Context<'a> {
	pub stage: Stage<'a>,
	pub storage: Storage,
	pub factory: GmoFactory,
	pub gmo_new_vec: Vec<GameObject>
}

fn process_game_objects(
    gmo_vec: &mut Vec<GameObject>, ctx: &mut Context
) {
    let mut i = 0;
    while i < gmo_vec.len() {
		let gmo = &mut gmo_vec[i];
    	gmo.bhv.update(&mut gmo.data, ctx, gmo.bhvd_index);
        let x = gmo.data.x;
        let y = gmo.data.y;
        if x < 0 || y < 0 || x as u32 >= ctx.stage.w || y as u32 >= ctx.stage.h {
			gmo.free(ctx);
            gmo_vec.swap_remove(i);
        } else {
			let sto = ctx.stage.get(gmo_vec[i].sto_index);
			sto.x = x;
			sto.y = y;
            i += 1;
        }
    }
    gmo_vec.append(&mut ctx.gmo_new_vec);
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
		gmo_new_vec: Vec::with_capacity(128)
	};

    let mut gmo_vec: Vec<GameObject> = Vec::new();

    let factory = ctx.factory;
	gmo_vec.push(
		factory.spawn_carrier(&mut ctx,
			GmoData { x: 700, y: 20 },
			BhvDataCarrier { dx: -3, interval: 30, cnt: 0 }
		)
	);
	gmo_vec.push(
		factory.spawn_carrier(&mut ctx,
			GmoData { x: 0, y: 40 },
			BhvDataCarrier { dx: 3, interval: 30, cnt: 0 }
		)
	);

	let mut evt_pump = sdl.event_pump().unwrap();
	let timer = sdl.timer().unwrap();
	let mut running = true;
	let mut next_tick: i32 = timer.ticks() as i32 + FPS_DELAY;

	while running {
    	ctx.stage.draw(&mut renderer);
		for evt in evt_pump.poll_iter() {
			match evt {
				Event::Quit { .. } => {
					running = false;
				},
				Event::KeyDown { keycode: Some(k), .. } => {
					let factory = ctx.factory;
					gmo_vec.push(
						factory.spawn_shot(
							&mut ctx,
							GmoData { x: 400, y: 599 },
							BhvDataMove { dx: 0, dy: -3 }
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

    	process_game_objects(&mut gmo_vec, &mut ctx);

		next_tick += FPS_DELAY;
	}
}
