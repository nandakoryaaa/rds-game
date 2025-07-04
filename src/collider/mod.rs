use crate::game::{ GmoType, GameObject };
use crate::pantry::Pantry;
use crate::Context;

//#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CollideGroup {
	NONE, SHOT, AERIAL, BOMB, FALLING, STANDING, GUN
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CollideMask {
	pub src: CollideGroup,
	pub dst: CollideGroup
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CollideStatus {
	NONE,		// не обрабатывать
	OFFSCREEN,	// за пределами экрана
	COLLIDE		// обрабатывать столкновение
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CollidePair {
	pub src_index: usize,
	pub dst_index: usize,
	pub status: CollideStatus
}

pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub w: u32,
	pub h: u32
}

pub struct Collider {}

impl Collider {
	pub fn check(
		&self, rect: Rect, pantry_gmo: &mut Pantry<GameObject>,
		vec_collide: &mut Vec<CollidePair>
	) {

		if pantry_gmo.len() == 0 {
			return;
		}

		let mut i = pantry_gmo.first_index();
		let last_index = pantry_gmo.last_index();
		loop {
			let mask = pantry_gmo.get(i).collide_mask;
			if mask.dst != CollideGroup::NONE || mask.src != CollideGroup::NONE
			{
				let src_data = pantry_gmo.get(i).data;
				if src_data.x as u32 >= rect.w	|| src_data.y as u32 >= rect.h
					|| src_data.x < rect.x || src_data.y < rect.y
				{
					vec_collide.push(
						CollidePair {
							src_index: i, dst_index: i,
							status: CollideStatus::OFFSCREEN
						}
					);
				} else {
					let mut j = i;
					while j != last_index {
						j = pantry_gmo.next_index(j);
						let gmo = pantry_gmo.get(j);
						if mask.dst == gmo.collide_mask.src	|| mask.src == gmo.collide_mask.dst {
							let dst_data = gmo.data;
							if src_data.x >= dst_data.x + (dst_data.w as i32)
								|| src_data.y >= dst_data.y + (dst_data.h as i32)
								|| src_data.x + (src_data.w as i32) < dst_data.x
								|| src_data.y + (src_data.h as i32) < dst_data.y
							{
								continue;
							}
							vec_collide.push(
								CollidePair { src_index: i, dst_index: j, status: CollideStatus::COLLIDE }
							);
						}
					}
				}
			}
			if i == last_index {
				break;
			}
			i = pantry_gmo.next_index(i);
		}
	}
}

pub struct SolverEvents {
	pub aerial_offscreen: u8,
	pub shot_carriers: u8,
	pub shot_bombers: u8,
	pub shot_chutes: u8,
	pub shot_falling: u8,
	pub shot_bombs: u8,
	pub blown_guns: u8,
	pub landed_left: u8,
	pub landed_right: u8,
	pub killed_left: u8,
	pub killed_right: u8
}

impl SolverEvents {
	pub fn new() -> Self {
		SolverEvents {
			aerial_offscreen: 0,
			shot_carriers: 0,
			shot_bombers: 0,
			shot_chutes: 0,
			shot_falling: 0,
			shot_bombs: 0,
			blown_guns: 0,
			landed_left: 0,
			landed_right: 0,
			killed_left: 0,
			killed_right: 0
		}
	}
}

pub struct Solver {}

impl Solver {
	pub fn solve(
		&self, pantry_gmo: &mut Pantry<GameObject>,
		vec_collide: &mut Vec<CollidePair>,
		ctx: &mut Context
	) -> SolverEvents {

		let mut sevt = SolverEvents::new();

		for i in 0..vec_collide.len() {
			let status = vec_collide[i].status;
			if status == CollideStatus::NONE {
				continue;
			}
			let src_index = vec_collide[i].src_index;
			let src_type = pantry_gmo.get(src_index).gmo_type;
			pantry_gmo.get(src_index).free(ctx);
			pantry_gmo.free(src_index);

			if status == CollideStatus::OFFSCREEN {
				if src_type == GmoType::CARRIER {
					sevt.aerial_offscreen += 1;
				}
			} else {
				let dst_index = vec_collide[i].dst_index;
				let dst_type = pantry_gmo.get(dst_index).gmo_type;
				pantry_gmo.get(dst_index).free(ctx);
				pantry_gmo.free(dst_index);
				if src_type == GmoType::CARRIER || dst_type == GmoType::CARRIER {
					sevt.shot_carriers += 1;
				} else if src_type == GmoType::BOMBER || dst_type == GmoType::BOMBER {
					sevt.shot_bombers += 1;
				} else if src_type == GmoType::CHUTE || dst_type == GmoType::CHUTE {
					sevt.shot_chutes += 1;
				}

				for j in i + 1..vec_collide.len() {
					let src_index2 = vec_collide[j].src_index;
					let dst_index2 = vec_collide[j].dst_index;
					if src_index == src_index2 || src_index == dst_index2
						|| dst_index == src_index2 || dst_index == dst_index2
					{
						vec_collide[j].status = CollideStatus::NONE;
					}
				}
			}
		}

		sevt
	}
}
