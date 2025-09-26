use crate::Context;
use crate::input::{ Input, InputEvent };
use crate::collider::{ Collider, CollidePair, Solver };
use crate::pantry::{ Pantry };
use crate::game::{ PlainRect, GmoType, StageObject, GameObject };
use crate::behaviour::{ BhvStatus, BhvDataMove, BhvDataGun };
use crate::static_drawable::*;
//use sdl2::keyboard::{Keycode};

#[derive (Copy, Clone, PartialEq)]
pub enum ControllerEvent {
	Run, EndTitle, EndGame
}

pub trait Controller
{
	fn begin(&mut self, ctx: &mut Context);
	fn run(&mut self, ctx: &mut Context, input: &Input) -> ControllerEvent;
	fn end(&mut self, ctx: &mut Context);
}

pub struct ControllerTitle {
	sto_logo_index: usize,
	step: i32,
	cnt: u32
}

impl ControllerTitle
{
	pub fn new() -> Self
	{
		Self {
			sto_logo_index: 0,
			step: 10,
			cnt: 0
		}
	}
}

impl Controller for ControllerTitle
{
	fn begin(&mut self, ctx: &mut Context)
	{
		ctx.stage.clear();
		self.sto_logo_index = ctx.stage.add_child(
			ctx.sto_factory.spawn_logo(0, 100)
		);
		self.step = 10;
		self.cnt = 5;
	}

	fn run(&mut self, ctx: &mut Context, input: &Input) -> ControllerEvent
	{
		if input.get_event() == InputEvent::Continue {
			return ControllerEvent::EndTitle;
		}

		let sto = ctx.stage.get_mut(self.sto_logo_index);
		sto.y += self.step;
		self.cnt -= 1;
		if self.cnt == 0 {
			self.cnt = 5;
			self.step = -self.step;
		}

		ControllerEvent::Run
	}

	fn end(&mut self, ctx: &mut Context)
	{
		ctx.stage.clear();
	}
}

pub struct ControllerGame {
	sto_shaft_index: usize,
	collider: Collider,
	solver: Solver,
	pantry_gmo: Pantry<GameObject>,
	vec_collide: Vec<CollidePair>,
	shoot_cooldown: u8,
	moving_dir: i8,
	shooting: bool
}

impl ControllerGame
{
	pub fn new(max_obj_cnt: usize) -> Self
	{
		Self {
			sto_shaft_index: 0,
			collider: Collider {},
			solver: Solver {},
			vec_collide: Vec::with_capacity(max_obj_cnt),
			pantry_gmo: Pantry::create(max_obj_cnt),
			shoot_cooldown: 0,
			moving_dir: 0,
			shooting: false
		}
	}
}

impl Controller for ControllerGame
{
	fn begin(&mut self, ctx: &mut Context)
	{
		let gmo_factory = ctx.gmo_factory;
		let mut gmo_gun = gmo_factory.spawn_gun(
			ctx, 384, 563,
			BhvDataGun { wave_type: GmoType::CARRIER, cnt: 10, delay: 30 }
		);
		gmo_gun.sto_index = ctx.stage.add_child(
			ctx.sto_factory.spawn_gun(gmo_gun.data.x, gmo_gun.data.y)
		);
		self.pantry_gmo.alloc(gmo_gun);
		self.sto_shaft_index = ctx.stage.add_child(
			StageObject { x: 400, y: 568, angle: 90, drawable: &DR_SHAFT }
		);
	}

	fn run(&mut self, ctx: &mut Context, input: &Input) -> ControllerEvent
	{
		let evt = input.get_event();

		if evt == InputEvent::Quit {
			return ControllerEvent::EndGame;
		}

		if evt == InputEvent::MoveLeft {
			self.moving_dir = 1;
		} else if evt == InputEvent::MoveRight {
			self.moving_dir = -1;
		} else if evt == InputEvent::Stop {
			self.moving_dir = 0;
		} else if evt == InputEvent::Shoot {
			self.shooting = true;
		} else if evt == InputEvent::StopShoot {
			self.shooting = false;
		}

		if self.moving_dir != 0 {
			let sto = ctx.stage.get_mut(self.sto_shaft_index);
			sto.angle += 3 * self.moving_dir as i32;
			if sto.angle > 180 {
				sto.angle = 180;
			} else if sto.angle < 0 {
				sto.angle = 0;
			}
		}

		if self.shoot_cooldown > 0 {
			self.shoot_cooldown -= 1;
		} else if self.shooting {
			self.shoot_cooldown = 10;
			let sto = ctx.stage.get(self.sto_shaft_index);
			let theta: f32 = (sto.angle as f32) * 3.1415926 / 180.0;
			let cos = theta.cos();
			let sin = theta.sin();
			let x = sto.x + (20.0 * cos).round() as i32;
			let y = sto.y - (20.0 * sin).round() as i32;

			let gmo_factory = ctx.gmo_factory;
			let mut gmo_shot = gmo_factory.spawn_shot(
				ctx, x, y,
				BhvDataMove {
					dx: (5.0 * cos).round() as i32,
					dy: -(5.0 * sin).round() as i32
				}
			);
			let sto_shot = ctx.sto_factory.spawn_shot(gmo_shot.data.x, gmo_shot.data.y);
			gmo_shot.sto_index = ctx.stage.add_child(sto_shot);
			self.pantry_gmo.alloc(gmo_shot);
		}

		// process_game_objects(&mut self.pantry_gmo, ctx);
		// process_game_objects inline
		if self.pantry_gmo.len() > 0 {
			let mut index = self.pantry_gmo.first_index();
			loop {
				// запомнить состояние последнего индекса,
				// т.к. после pantry_gmo.free() он может измениться
				let is_last = self.pantry_gmo.is_last_index(index);
				let gmo = self.pantry_gmo.get_mut(index);
				let status = gmo.bhv.update(ctx, gmo);
				if status == BhvStatus::END {
					gmo.free(ctx);
					self.pantry_gmo.free(index);
				} else {
					let sto = ctx.stage.get_mut(gmo.sto_index);
					sto.x = gmo.data.x;
					sto.y = gmo.data.y;
				}
				if is_last {
					break;
				}
				index = self.pantry_gmo.next_index(index);
			}
		}

		self.collider.check(
			PlainRect { x: 0, y: 0, w: ctx.stage.w, h: ctx.stage.h },
			&mut self.pantry_gmo,
			&mut self.vec_collide
		);

		if self.vec_collide.len() > 0 {
			let sevt = self.solver.solve(&mut self.pantry_gmo, &mut self.vec_collide, ctx);
			let score = 10 * sevt.shot_carriers + 5 * sevt.shot_chutes;
			if score > 0 {
				//println!("score: {}", score);
			}
			self.vec_collide.clear();
		}

		while ctx.vec_gmo_new.len() > 0 {
			let mut new = ctx.vec_gmo_new.pop().unwrap();
			new.gmo.sto_index = ctx.stage.add_child(new.sto);
			self.pantry_gmo.alloc(new.gmo);
		}

		ControllerEvent::Run
	}

	fn end(&mut self, ctx: &mut Context)
	{
		ctx.stage.clear();
		self.pantry_gmo.clear();
		self.vec_collide.clear();
	}
}
