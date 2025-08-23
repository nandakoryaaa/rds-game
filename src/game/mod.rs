use crate::renderer::Drawable;
use crate::behaviour::Behaviour;
use crate::Context;
use crate::pantry::*;
use crate::renderer::*;
use crate::collider::*;

#[derive(Copy, Clone)]
pub struct PlainRect {
	pub x: i32,
	pub y: i32,
	pub w: u32,
	pub h: u32
}

pub struct StageObject {
	pub x: i32,
	pub y: i32,
	pub angle: i32,
	pub drawable: &'static dyn Drawable
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
	FALLING, STANDING, SPLOSH, BOMB, EXPLOSION
}

pub struct GameObject {
	pub gmo_type: GmoType,
	pub data: GmoData,
	pub collide_mask: CollideMask, 
	pub bhv: &'static dyn Behaviour,
	pub sto_index: usize,
	pub bhvd_index: usize
}

impl GameObject {
	pub fn update_from(
		&mut self, ctx: &mut Context,
		gmo: &GameObject, sto: StageObject
	) {
		self.bhv.free(ctx, self.bhvd_index);
		self.bhv = gmo.bhv;
		self.bhvd_index = gmo.bhvd_index;
		self.gmo_type = gmo.gmo_type;
		self.data = gmo.data;
		self.collide_mask = gmo.collide_mask;
		ctx.stage.update_child(self.sto_index, sto);
	}

	pub fn free(&self, ctx: &mut Context) {
		ctx.stage.remove_child(self.sto_index);
		self.bhv.free(ctx, self.bhvd_index);
	}
}

pub struct GmoNew {
	pub sto: StageObject,
	pub gmo: GameObject
}

pub struct Stage {
	pub w: u32,
	pub h: u32,
	pub pantry_sto: Pantry<StageObject>
}

impl Stage {
	pub fn add_child(&mut self, sto: StageObject) -> usize {
		return self.pantry_sto.alloc(sto);
	}

	pub fn update_child(&mut self, index: usize, sto: StageObject) {
		self.pantry_sto.update(index, sto);
	}

	pub fn remove_child(&mut self, index: usize) {
		self.pantry_sto.free(index);
	}

	pub fn get(&mut self, index: usize) -> &StageObject {
		self.pantry_sto.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> &mut StageObject {
		self.pantry_sto.get_mut(index)
	}

	pub fn draw(&self, renderer: &mut Renderer) {
		if self.pantry_sto.len() > 0 {
			let mut index = self.pantry_sto.first_index();
	   		loop {
				let is_last = self.pantry_sto.is_last_index(index);
				let sto = self.pantry_sto.get(index);
				sto.drawable.draw(sto, renderer);
				if is_last {
					break;
				}
				index = self.pantry_sto.next_index(index);
			}
		}
	}
}
