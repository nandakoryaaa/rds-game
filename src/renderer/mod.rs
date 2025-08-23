extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, Texture };
use crate::game::{ PlainRect, StageObject };

pub struct Renderer<'a> {
	pub canvas: &'a mut WindowCanvas,
	pub texture_list: Vec<Texture<'a>>
}

impl<'a> Renderer<'a> {
	pub fn clear(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		let _ = self.canvas.fill_rect(None);
	}

	pub fn present(&mut self) {
		self.canvas.present();
	}

	pub fn draw_bitmap(
		&mut self, x:i32, y:i32, tex_handle: usize, rect: PlainRect
	) {
		self.canvas.copy(
			&self.texture_list[tex_handle],
			Some(Rect::new(rect.x, rect.y, rect.w, rect.h)),
			Some(Rect::new(x, y, rect.w, rect.h))
		);
	}

	pub fn draw_rect(&mut self, x:i32, y:i32, w:u32, h:u32, color:Color) {
		self.canvas.set_draw_color(color);
		let _ = self.canvas.fill_rect(Rect::new(x, y, w, h));
	}

	pub fn rasterize_poly(
		&mut self,
		mut x0:i32, mut y0:i32, mut x1:i32, mut y1:i32,
		mut x2:i32, mut y2:i32, color:Color
	) {
    	if y0 > y1 {
        	let mut tmp = y0;
        	y0 = y1;
        	y1 = tmp;
        	tmp = x0;
        	x0 = x1;
        	x1 = tmp;
    	}

    	if y0 > y2 {
        	let mut tmp = y0;
        	y0 = y2;
        	y2 = tmp;
        	tmp = x0;
        	x0 = x2;
        	x2 = tmp;
    	}

    	if y1 > y2 {
        	let mut tmp = y1;
        	y1 = y2;
        	y2 = tmp;
        	tmp = x1;
        	x1 = x2;
        	x2 = tmp;
    	}

	    let mut dx1 = x1 - x0;
    	let mut dy1 = y1 - y0;
	    let dx2 = x2 - x0;
	    let dy2 = y2 - y0;

    	let mut top_y = y0;

	    while top_y < y1 {
    	    let cross_x1 = x0 + dx1 * (top_y - y0) / dy1;
	        let cross_x2 = x0 + dx2 * (top_y - y0) / dy2;
	        if cross_x1 > cross_x2 {
				self.draw_rect(cross_x2, top_y, (cross_x1 - cross_x2) as u32, 1, color);
	        } else {
	            self.draw_rect(cross_x1, top_y, (cross_x2 - cross_x1) as u32, 1, color);
	        }
	        top_y += 1;
    	}

	    dx1 = x2 - x1;
	    dy1 = y2 - y1;
	    while top_y < y2 {
    	    let cross_x1 = x1 + dx1 * (top_y - y1) / dy1;
	        let cross_x2 = x0 + dx2 * (top_y - y0) / dy2;
    	    if cross_x1 > cross_x2 {
        	    self.draw_rect(cross_x2, top_y, (cross_x1 - cross_x2) as u32, 1, color);
        	} else {
            	self.draw_rect(cross_x1, top_y, (cross_x2 - cross_x1) as u32, 1, color);
        	}
	        top_y += 1;
    	}
	}
}

pub trait Drawable {
	fn draw(&self, sto: &StageObject, renderer: &mut Renderer);
}

pub struct DrawableBitmap {
	pub tex_handle: usize,
	pub rect: PlainRect
}

pub struct DrawableRect {
	pub color: Color,
	pub w: u32,
	pub h: u32
}

pub struct DrawableRotRect {
	pub color: Color,
	pub cx: i32,
	pub cy: i32,
	pub w: u32,
	pub h: u32
}

#[derive(Copy, Clone)]
pub struct ColorRect {
	pub x: i32,
	pub y: i32,
	pub w: u32,
	pub h: u32,
	pub color: Color
}

pub struct DrawableListRect {
	pub cnt: usize,
	pub list_rect: [ColorRect; 5]
}

impl Drawable for DrawableBitmap {
	fn draw(&self, sto: &StageObject, renderer: &mut Renderer) {
		renderer.draw_bitmap(sto.x, sto.y, self.tex_handle, self.rect);
	}
}

impl Drawable for DrawableRect {
	fn draw(&self, sto: &StageObject, renderer: &mut Renderer) {
		renderer.draw_rect(sto.x, sto.y, self.w, self.h, self.color);
	}
}

impl Drawable for DrawableListRect {
	fn draw(&self, sto: &StageObject, renderer: &mut Renderer) {
		for i in 0..self.cnt {
			let rect = self.list_rect[i];
			renderer.draw_rect(sto.x + rect.x, sto.y + rect.y, rect.w, rect.h, rect.color);
		}
	}
}

impl Drawable for DrawableRotRect {
	fn draw(&self, sto: &StageObject, renderer: &mut Renderer) {
		let theta: f32 = (sto.angle as f32) * 3.1415926 / 180.0;
		let cos = theta.cos();
		let sin = theta.sin();

		let cx = self.cx;
		let cy = self.cy;
		let x0 = -cx;
		let y0 = cy;
		let mut x1 = self.w as i32 - cx;
		let mut y1 = cy;
		let x2 = self.w as i32 - cx;
		let y2 = cy - self.h as i32;

		let cxx = sto.x;
		let cyy = sto.y;

		renderer.rasterize_poly(
			cxx + (x0 as f32 * cos - y0 as f32 * sin).round() as i32,
			cyy - (x0 as f32 * sin + y0 as f32 * cos).round() as i32,
			cxx + (x1 as f32 * cos - y1 as f32 * sin).round() as i32,
			cyy - (x1 as f32 * sin + y1 as f32 * cos).round() as i32,
			cxx + (x2 as f32 * cos - y2 as f32 * sin).round() as i32,
			cyy - (x2 as f32 * sin + y2 as f32 * cos).round() as i32,
			self.color
		);
		
		x1 = x0;
		y1 = y2;

		renderer.rasterize_poly(
			cxx + (x0 as f32 * cos - y0 as f32 * sin).round() as i32,
			cyy - (x0 as f32 * sin + y0 as f32 * cos).round() as i32,
			cxx + (x1 as f32 * cos - y1 as f32 * sin).round() as i32,
			cyy - (x1 as f32 * sin + y1 as f32 * cos).round() as i32,
			cxx + (x2 as f32 * cos - y2 as f32 * sin).round() as i32,
			cyy - (x2 as f32 * sin + y2 as f32 * cos).round() as i32,
			self.color
		);
	}
}
