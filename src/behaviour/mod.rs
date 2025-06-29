use crate::game::{ GmoData };
use crate::Context;

pub struct BhvDataCarrier {
	pub dx: i32,
	pub interval: u32,
	pub cnt: u32
}

pub struct BhvDataMove {
	pub dx: i32,
	pub dy: i32
}

pub trait Behaviour {
	fn update(
		&self, gmo_data: &mut GmoData,
		ctx: &mut Context, index: usize
	);
	fn free(&self, ctx: &mut Context, index: usize);
}

pub struct BehaviourCarrier {}

impl Behaviour for BehaviourCarrier {
	fn update(
		&self, gmo_data: &mut GmoData, ctx: &mut Context, index: usize
	) {
		let bhv_data = ctx.storage.pantry_bhvd_carrier.get(index);
		gmo_data.x += bhv_data.dx;
		bhv_data.cnt += 1;
		if bhv_data.cnt == bhv_data.interval {
			bhv_data.cnt = 0;
			let factory = ctx.factory;
			let gmo = factory.spawn_chute(
				ctx,
				GmoData { x: gmo_data.x, y: gmo_data.y, w:10, h:10 },
				BhvDataMove { dx: 0, dy: 2 }
			);

			ctx.vec_gmo_new.push(gmo);
		}
	}

	fn free(&self, ctx: &mut Context, index: usize) {
		ctx.storage.pantry_bhvd_carrier.free(index);
	}
}

pub struct BehaviourMove {}

impl Behaviour for BehaviourMove {
	fn update(
		&self, gmo_data: &mut GmoData, ctx: &mut Context, index: usize
	) {
		let bhv_data = ctx.storage.pantry_bhvd_move.get(index);
		gmo_data.x += bhv_data.dx;
		gmo_data.y += bhv_data.dy;
	}

	fn free(&self, ctx: &mut Context, index: usize) {
		ctx.storage.pantry_bhvd_move.free(index);
	}
}
