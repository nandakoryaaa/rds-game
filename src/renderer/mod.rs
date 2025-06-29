extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Renderer<'a> {
	pub canvas: &'a mut WindowCanvas
}

impl<'a> Renderer<'a> {
	pub fn clear(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		let _ = self.canvas.fill_rect(None);
	}

	pub fn present(&mut self) {
		self.canvas.present();
	}

	pub fn draw_bitmap(&mut self, x:i32, y:i32, handle:u32) {
		println!("drawing Bitmap {}", handle);
	}

	pub fn draw_rect(&mut self, x:i32, y:i32, w:u32, h:u32, color:Color) {
		self.canvas.set_draw_color(color);
		let _ = self.canvas.fill_rect(Rect::new(x, y, w, h));
	}
}

pub trait Drawable {
	fn draw(&self, x:i32, y:i32, renderer: &mut Renderer);
}

pub struct DrawableBitmap {
	pub handle: u32
}

pub struct DrawableRect {
	pub color: Color,
	pub w: u32,
	pub h: u32
}

impl Drawable for DrawableBitmap {
	fn draw(&self, x:i32, y:i32, renderer: &mut Renderer) {
		renderer.draw_bitmap(x, y, self.handle);
	}
}

impl Drawable for DrawableRect {
	fn draw(&self, x:i32, y:i32, renderer: &mut Renderer) {
		renderer.draw_rect(x, y, self.w, self.h, self.color);
	}
}
