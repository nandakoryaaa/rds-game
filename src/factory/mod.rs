use crate::game::*;
use crate::behaviour::*;
use crate::collider::*;
use crate::{ Context, DR_SHOT, DR_CARRIER, DR_BOMBER, DR_CHUTE, DR_BOMB };

#[derive(Copy, Clone)]
pub struct GmoFactory {
}

impl GmoFactory {
	pub fn spawn_shot(
		&self, ctx: &mut Context, gmo_data: GmoData, bhv_data: BhvDataMove
	) -> GameObject {
		GameObject {
			sto_index: ctx.stage.add_child(
				StageObject {
					x: gmo_data.x, y: gmo_data.y, 
					drawable: &DR_SHOT
				}
			),
			data: gmo_data,
			collide_mask: CollideMask {
				src: CollideGroup::SHOT, dst: CollideGroup::AERIAL
			},
			bhv: &BehaviourMove {},
			bhvd_index: ctx.storage.pantry_bhvd_move.alloc(bhv_data)
		}
	}

	pub fn spawn_carrier(
		&self, ctx: &mut Context, gmo_data: GmoData, bhv_data: BhvDataCarrier
	) -> GameObject {
		GameObject {
			sto_index: ctx.stage.add_child(
				StageObject {
					x: gmo_data.x, y: gmo_data.y,
					drawable: &DR_CARRIER
				}
			),
			data: gmo_data,
			collide_mask: CollideMask {
				src: CollideGroup::AERIAL, dst: CollideGroup::NONE
			},
			bhv: &BehaviourCarrier {},
			bhvd_index: ctx.storage.pantry_bhvd_carrier.alloc(bhv_data)
		}
	}

	pub fn spawn_chute(
		&self, ctx: &mut Context, gmo_data: GmoData, bhv_data: BhvDataMove
	) -> GameObject {
		GameObject {
			sto_index: ctx.stage.add_child(
				StageObject {
					x: gmo_data.x, y: gmo_data.y,
					drawable: &DR_CHUTE
				}
			),
			data: gmo_data,
			collide_mask: CollideMask {
				src: CollideGroup::AERIAL, dst: CollideGroup::NONE
			},
			bhv: &BehaviourMove {},
			bhvd_index: ctx.storage.pantry_bhvd_move.alloc(bhv_data)
		}
	}
}

