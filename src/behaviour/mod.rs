use crate::game::{ GmoType, GameObject, GmoNew };
use crate::Context;

#[derive(PartialEq, Eq)]
pub enum BhvStatus {
	OK, END
}

pub struct BhvDataMove {
	pub dx: i32,
	pub dy: i32
}

pub struct BhvDataGun {
	pub wave_type: GmoType,
	pub cnt: u32,
	pub delay: u32
}

pub struct BhvDataTimedMotion {
	pub speed: i32,
	pub delay: u32
}

pub trait Behaviour
{
	fn update(
		&self, ctx: &mut Context, gmo: &mut GameObject
	) -> BhvStatus;

	fn free(&self, ctx: &mut Context, index: usize);
}

pub struct BehaviourMove {}

impl Behaviour for BehaviourMove {	// safe
	fn update(
		&self, ctx: &mut Context, gmo: &mut GameObject
	) -> BhvStatus {
		let bhv_data = ctx.storage.pantry_bhvd_move.get(gmo.bhvd_index);
		gmo.data.x += bhv_data.dx;
		gmo.data.y += bhv_data.dy;
		BhvStatus::OK
	}

	fn free(&self, ctx: &mut Context, index: usize) {
		ctx.storage.pantry_bhvd_move.free(index);
	}
}

pub struct BehaviourGravityMove {}

impl Behaviour for BehaviourGravityMove {	// safe
	fn update(
		&self, ctx: &mut Context, gmo: &mut GameObject
	) -> BhvStatus {
		let bhv_data = ctx.storage.pantry_bhvd_tm.get_mut(gmo.bhvd_index);
		gmo.data.y += bhv_data.speed;
		bhv_data.speed += 1;
		if gmo.data.y < ctx.stage.h as i32 {
			return BhvStatus::OK;
		}
		BhvStatus::END
	}

	fn free(&self, ctx: &mut Context, index: usize) {
		ctx.storage.pantry_bhvd_tm.free(index);
	}
}

pub struct BehaviourGun {}

impl Behaviour for BehaviourGun {	// safe - does not replace gmo
	fn update(
		&self, ctx: &mut Context, gmo: &mut GameObject
	) -> BhvStatus {
		let bhv_data = ctx.storage.pantry_bhvd_gun.get_mut(gmo.bhvd_index);
		if bhv_data.cnt > 0 {
			if bhv_data.delay > 0 {
				bhv_data.delay -= 1;
			} else {
				//bhv_data.cnt -= 1;
				bhv_data.delay = ctx.rand.randint(100, 500);
				let mut dx = 1;
				let mut x = 0;
				if ctx.rand.randint(0, 2) == 1 {
					dx = -1;
					x = 700;
				}
				let delay = ctx.rand.randint(10, 20);
				let gmo_factory = ctx.gmo_factory;
				let gmc = gmo_factory.spawn_carrier(
					ctx, x, 40,	BhvDataTimedMotion { speed: dx, delay: delay }
				);
				ctx.vec_gmo_new.push(
					GmoNew {
						sto: ctx.sto_factory.spawn_carrier(gmc.data.x, gmc.data.y, dx),
						gmo: gmc
					}
				);
			}
		}
		BhvStatus::OK
	}

	fn free(&self, ctx: &mut Context, index: usize) {
		ctx.storage.pantry_bhvd_gun.free(index);
	}
}

pub struct BehaviourCarrier {}

impl Behaviour for BehaviourCarrier {	// safe - does not replace gmo
	fn update(
		&self, ctx: &mut Context, gmo: &mut GameObject
	) -> BhvStatus {
		let bhv_data = ctx.storage.pantry_bhvd_tm.get_mut(gmo.bhvd_index);
		let speed = bhv_data.speed;
		if bhv_data.delay > 0 {
			bhv_data.delay -= 1;
		} else {
			bhv_data.delay = ctx.rand.randint(50, 100);
			let gmo_factory = ctx.gmo_factory;
			let mut gmt = gmo_factory.spawn_trooper(ctx, gmo.data.x, gmo.data.y + 24);
			if speed > 0 {
				gmt.data.x += 54;
			} else {
				gmt.data.x += 31;
			}
			ctx.vec_gmo_new.push(
				GmoNew {
					sto: ctx.sto_factory.spawn_trooper(gmt.data.x, gmt.data.y),
					gmo: gmt
				}
			);
		}
		gmo.data.x += speed;
		BhvStatus::OK
	}

	fn free(&self, ctx: &mut Context, index: usize) {
		ctx.storage.pantry_bhvd_tm.free(index);
	}
}

pub struct BehaviourTrooper {}

impl Behaviour for BehaviourTrooper {	// replaces gmo
	fn update(
		&self, ctx: &mut Context, gmo: &mut GameObject
	) -> BhvStatus {
		let bhv_data = ctx.storage.pantry_bhvd_tm.get_mut(gmo.bhvd_index);
		gmo.data.y += bhv_data.speed;
		if bhv_data.delay > 0 {
			bhv_data.delay -= 1;
			if bhv_data.delay == 0 {
				// need to replace trooper with chute
				let gmo_factory = ctx.gmo_factory;
				let mut gmc = gmo_factory.spawn_chute(ctx, gmo.data.x, gmo.data.y);
				gmc.data.y -= gmc.data.h - gmo.data.h;
				gmc.data.x -= ((gmc.data.w - gmo.data.w) >> 1) as i32;
				let sto = ctx.sto_factory.spawn_chute(gmc.data.x, gmc.data.y);
				// in-place
				gmo.update_from(ctx, &gmc, sto);
			}
		}
		BhvStatus::OK
	}

	fn free(&self, ctx: &mut Context, index: usize) {
		ctx.storage.pantry_bhvd_tm.free(index);
	}
}
