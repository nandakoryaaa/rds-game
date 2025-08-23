use sdl2::pixels::Color;

use crate::renderer::{ DrawableBitmap, DrawableRect, DrawableListRect, DrawableRotRect, ColorRect };
use crate::game::PlainRect;

pub static DR_SHOT: DrawableRect = DrawableRect {
	w: 3, h: 3, color: Color::RGB(255, 255, 255)
};

pub static DR_SPLOSH: DrawableRect = DrawableRect {
	w: 5, h: 5, color: Color::RGB(255, 0, 0)
};

pub static DR_SHAFT: DrawableRotRect = DrawableRotRect {
	w:20, h:6, cx: 0, cy: 3, color: Color::RGB(0, 0, 255)
};

pub static DR_CARRIER_LEFT: DrawableBitmap = DrawableBitmap {
	tex_handle: 0,
	rect: PlainRect { x: 0, y: 188, w: 101, h: 50 }
};

pub static DR_CARRIER_RIGHT: DrawableBitmap = DrawableBitmap {
	tex_handle: 0,
	rect: PlainRect { x: 102, y: 188, w: 101, h: 50 }
};

pub static DR_GUN: DrawableListRect = DrawableListRect {
	cnt: 4,
	list_rect: [
		ColorRect { x: 13, y: 0, w: 6, h: 2, color: Color::RGB(0, 0, 255) },
		ColorRect { x: 10, y: 2, w: 12, h: 3, color: Color::RGB(0, 0, 255) },
		ColorRect { x: 8, y: 5, w: 16, h: 6, color: Color::RGB(0, 0, 255) },
		ColorRect { x: 0, y: 11, w: 32, h: 26, color: Color::RGB(0, 0, 255) },
		ColorRect { x: 0, y: 0, w: 0, h: 0, color: Color::RGB(0, 0, 0) }
	]
};

pub static DR_BOMBER: DrawableRect = DrawableRect {
	w: 30, h: 20, color: Color::RGB(0, 0, 255)
};

pub static DR_TROOPER: DrawableBitmap = DrawableBitmap {
	tex_handle: 0,
	rect: PlainRect { x: 246, y: 188, w: 17, h: 26 }
};

pub static DR_CHUTE: DrawableBitmap = DrawableBitmap {
	tex_handle: 0,
	rect: PlainRect { x: 204, y: 188, w: 41, h: 51 }
};

pub static DR_FALLING: DrawableBitmap = DrawableBitmap {
	tex_handle: 0,
	rect: PlainRect { x: 264, y: 188, w: 16, h: 25 }
};


pub static DR_BOMB: DrawableRect = DrawableRect {
	w: 10, h: 10, color: Color::RGB(255, 0, 0)
};
