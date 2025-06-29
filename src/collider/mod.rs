use crate::game::GameObject;
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

pub struct CollidePair {
	pub src_index: usize,
	pub dst_index: usize,
	pub status: u8
}

pub struct Collider {}

impl Collider {
	pub fn check(
		&self, pantry_gmo: &mut Pantry<GameObject>,
		vec_collide: &mut Vec<CollidePair>
	) {

		if pantry_gmo.len() == 0 {
			return;
		}

		let mut i = pantry_gmo.first_index();
		let last_index = pantry_gmo.last_index();
		loop {
			let mask = pantry_gmo.get(i).collide_mask;
			if mask.dst != CollideGroup::NONE
				|| mask.src != CollideGroup::NONE
			{
				let src_data = pantry_gmo.get(i).data;
				let mut j = i;
				while j != last_index {
					j = pantry_gmo.next_index(j);
					let gmo = pantry_gmo.get(j);
					if mask.dst == gmo.collide_mask.src
						|| mask.src == gmo.collide_mask.dst
					{
						let dst_data = gmo.data;
						if src_data.x >= dst_data.x + (dst_data.w as i32)
							|| src_data.y >= dst_data.y + (dst_data.h as i32)
							|| src_data.x + (src_data.w as i32) < dst_data.x
							|| src_data.y + (src_data.h as i32) < dst_data.y
						{
							continue;
						}
						vec_collide.push(
							CollidePair { src_index: i, dst_index: j, status: 1 }
						);
						println!("collision! {} {}", i, j);
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

pub struct Solver {}

impl Solver {
	pub fn solve(
		&self, pantry_gmo: &mut Pantry<GameObject>,
		vec_collide: &mut Vec<CollidePair>,
		ctx: &mut Context
	) {
		for i in 0..vec_collide.len() {
			if (vec_collide[i].status == 0) {
				continue;
			}
			let src_index = vec_collide[i].src_index;
			let dst_index = vec_collide[i].dst_index;

			let mut gmo = pantry_gmo.get(src_index);
			gmo.free(ctx);
			pantry_gmo.free(src_index);
			gmo = pantry_gmo.get(dst_index);
			gmo.free(ctx);
			pantry_gmo.free(dst_index);

			for j in i + 1..vec_collide.len() {
				let src_index2 = vec_collide[j].src_index;
				let dst_index2 = vec_collide[j].dst_index;
				if src_index == src_index2 || src_index == dst_index2
					|| dst_index == src_index2 || dst_index == dst_index2
				{
					vec_collide[j].status = 0;
				}
			}
		}
 	}
}
