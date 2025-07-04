use crate::renderer::Drawable;
use crate::behaviour::Behaviour;
use crate::Context;
use crate::pantry::*;
use crate::renderer::*;
use crate::collider::*;


pub struct StageObject<'a> {
	pub x: i32,
	pub y: i32,
	pub drawable: &'a dyn Drawable,
}

#[derive(Copy, Clone)]
pub struct GmoData {
	pub x: i32,
	pub y: i32,
	pub w: u32,
	pub h: i32
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GmoType {
	NONE, GUN, SHOT, CARRIER, BOMBER, CHUTE,
	FALLING, STANDING, BOMB, EXPLOSION
}

pub struct GameObject {
	pub gmo_type: GmoType,
	pub data: GmoData,
	pub collide_mask: CollideMask, 
	pub sto_index: usize,
	pub bhv: &'static dyn Behaviour,
	pub bhvd_index: usize
}

impl GameObject {
	pub fn free(&mut self, ctx: &mut Context) {
		ctx.stage.remove_child(self.sto_index);
		self.bhv.free(ctx, self.bhvd_index);
	}
}

pub struct Stage<'a> {
	pub w: u32,
	pub h: u32,
	pub pantry_sto: Pantry<StageObject<'a>>
}

impl<'a> Stage<'a> {
	pub fn add_child(&mut self, sto: StageObject<'a>) -> usize {
		return self.pantry_sto.alloc(sto);
	}

	pub fn remove_child(&mut self, index: usize) {
		self.pantry_sto.free(index);
	}

	pub fn get(&mut self, index: usize) -> &mut StageObject<'a> {
		self.pantry_sto.get(index)
	}
	
	pub fn draw(&self, renderer: &mut Renderer) {
		renderer.clear();
		if self.pantry_sto.used_cnt > 0 {
			let mut index = self.pantry_sto.used_first;
	   		loop {
				let sto = &self.pantry_sto.entries[index].payload;
				sto.drawable.draw(sto.x, sto.y, renderer);
				if index == self.pantry_sto.used_last {
					break;
				}
				index = self.pantry_sto.entries[index].next;
			}
		}
		renderer.present();
	}
}
