use crate::game::*;
use crate::behaviour::*;
use crate::collider::*;
use crate::{ Context };
use crate::static_drawable::*;

#[derive(Copy, Clone)]
pub struct GmoFactory {
}

#[derive(Copy, Clone)]
pub struct StoFactory {
}

impl GmoFactory
{
	pub fn spawn_shot(
		&self, ctx: &mut Context, x: i32, y: i32, bhv_data: BhvDataMove
	) -> GameObject {
		GameObject {
			gmo_type: GmoType::SHOT,
			sto_index: 0,
			data: GmoData { x: x, y: y, w: 3, h: 3 },
			collide_mask: CollideMask {
				src: CollideGroup::SHOT, dst: CollideGroup::AERIAL
			},
			bhv: &BehaviourMove {},
			bhvd_index: ctx.storage.pantry_bhvd_move.alloc(bhv_data)
		}
	}

	pub fn spawn_carrier(
		&self, ctx: &mut Context, x: i32, y: i32, bhv_data: BhvDataTimedMotion
	) -> GameObject {
		GameObject {
			gmo_type: GmoType::CARRIER,
			sto_index: 0,
			data: GmoData { x: x, y: y, w: 101, h: 50 },
			collide_mask: CollideMask {
				src: CollideGroup::AERIAL, dst: CollideGroup::NONE
			},
			bhv: &BehaviourCarrier {},
			bhvd_index: ctx.storage.pantry_bhvd_tm.alloc(bhv_data)
		}
	}

	pub fn spawn_gun(
		&self, ctx: &mut Context, x: i32, y: i32, bhv_data: BhvDataGun
	) -> GameObject {
		GameObject {
			gmo_type: GmoType::GUN,
			sto_index: 0,
			data: GmoData { x: x, y: y, w: 32, h: 37 },
			collide_mask: CollideMask {
				src: CollideGroup::GUN, dst: CollideGroup::NONE
			},
			bhv: &BehaviourGun {},
			bhvd_index: ctx.storage.pantry_bhvd_gun.alloc(bhv_data)
		}
	}

	pub fn spawn_trooper(&self, ctx: &mut Context, x: i32, y: i32) -> GameObject
	{
		let delay = ctx.rand.randint(10, 40);
		let bhvd_index = ctx.storage.pantry_bhvd_tm.alloc(
			BhvDataTimedMotion { speed: 6, delay: delay }
		);
		GameObject {
			gmo_type: GmoType::FALLING,
			sto_index: 0,
			data: GmoData { x: x, y: y, w: 17, h: 26 },
			collide_mask: CollideMask {
				src: CollideGroup::AERIAL, dst: CollideGroup::NONE
			},
			bhv: &BehaviourTrooper {},
			bhvd_index: bhvd_index
		}
	}

	pub fn spawn_falling(&self, ctx: &mut Context, x: i32, y: i32) -> GameObject
	{
		GameObject {
			gmo_type: GmoType::FALLING,
			sto_index: 0,
			data: GmoData { x: x, y: y, w: 16, h: 25 },
			collide_mask: CollideMask {
				src: CollideGroup::AERIAL, dst: CollideGroup::NONE
			},
			bhv: &BehaviourMove {},
			bhvd_index: ctx.storage.pantry_bhvd_move.alloc(
				BhvDataMove { dx: 0, dy: 6 }
			)
		}
	}

	pub fn spawn_chute(&self, ctx: &mut Context, x: i32, y: i32) -> GameObject
	{
		GameObject {
			gmo_type: GmoType::CHUTE,
			sto_index: 0,
			data: GmoData { x: x, y: y, w: 41, h: 51 },
			collide_mask: CollideMask {
				src: CollideGroup::AERIAL, dst: CollideGroup::NONE
			},
			bhv: &BehaviourMove {},
			bhvd_index: ctx.storage.pantry_bhvd_move.alloc(
				BhvDataMove { dx: 0, dy: 3 }
			)
		}
	}

	pub fn spawn_splosh(&self, ctx: &mut Context, x: i32, y: i32, speed: i32) -> GameObject
	{
		GameObject {
			gmo_type: GmoType::SPLOSH,
			sto_index: 0,
			data: GmoData { x: x, y: y, w: 0, h: 0 },
			collide_mask: CollideMask {
				src: CollideGroup::NONE, dst: CollideGroup::NONE
			},
			bhv: &BehaviourGravityMove {},
			bhvd_index: ctx.storage.pantry_bhvd_tm.alloc(
				BhvDataTimedMotion { speed: speed, delay: 0 }
			)
		}
	}
}

impl StoFactory
{
	pub fn spawn_logo(&self, x: i32, y: i32) -> StageObject
	{
		StageObject { x: x, y: y, angle: 0, drawable: &DR_LOGO }
	}

	pub fn spawn_shot(&self, x: i32, y: i32) -> StageObject
	{
		StageObject { x: x, y: y, angle: 0, drawable: &DR_SHOT }
	}

	pub fn spawn_splosh(&self, x: i32, y: i32) -> StageObject
	{
		StageObject { x: x, y: y, angle: 0, drawable: &DR_SPLOSH }
	}

	pub fn spawn_carrier(&self, x: i32, y: i32, speed: i32) -> StageObject
	{
		StageObject {
			x: x, y: y, angle: 0,
			drawable: if speed < 0 { &DR_CARRIER_LEFT } else { &DR_CARRIER_RIGHT }
		}
	}

	pub fn spawn_gun(&self, x: i32, y:i32) -> StageObject
	{
		StageObject { x: x, y: y, angle: 0, drawable: &DR_GUN }
	}

	pub fn spawn_trooper(&self, x: i32, y: i32) -> StageObject
	{
		StageObject { x: x, y: y,  angle: 0, drawable: &DR_TROOPER }
	}

	pub fn spawn_falling(&self, x: i32, y: i32) -> StageObject
	{
		StageObject { x: x, y: y,  angle: 0, drawable: &DR_FALLING }
	}

	pub fn spawn_chute(&self, x: i32, y: i32) -> StageObject
	{
		StageObject { x: x, y: y, angle: 0, drawable: &DR_CHUTE }
	}
}
