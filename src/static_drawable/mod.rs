use sdl2::pixels::Color;
use crate::renderer::{ DrawableRect, DrawableListRect, DrawableRotRect, ColorRect };

pub static DR_SHOT: DrawableRect = DrawableRect {
	w: 3, h: 3, color: Color::RGB(255, 255, 255)
};

pub static DR_SPLOSH: DrawableRect = DrawableRect {
	w: 5, h: 5, color: Color::RGB(255, 0, 0)
};

pub static DR_SHAFT: DrawableRotRect = DrawableRotRect {
	w:20, h:6, cx: 0, cy: 3, color: Color::RGB(0, 0, 255)
};

pub static DR_CARRIER_LEFT: DrawableListRect = DrawableListRect {
	cnt: 5,
	list_rect: [
		ColorRect { x: 0, y: 0, w: 50, h: 4, color: Color::RGB(255, 255, 0) },
		ColorRect { x: 57, y: 3, w: 6, h: 4, color: Color::RGB(255, 255, 0) },
		ColorRect { x: 17, y: 7, w: 45, h: 4, color: Color::RGB(255, 255, 0) },
		ColorRect { x: 12, y: 11, w: 24, h: 8, color: Color::RGB(255, 255, 0) },
		ColorRect { x: 15, y: 19, w: 16, h: 4, color: Color::RGB(255, 255, 0) },
	]
};

pub static DR_CARRIER_RIGHT: DrawableListRect = DrawableListRect {
	cnt: 5,
	list_rect: [
		ColorRect { x: 13, y: 0, w: 50, h: 4, color: Color::RGB(255, 255, 0) },
		ColorRect { x: 0, y: 3, w: 6, h: 4, color: Color::RGB(255, 255, 0) },
		ColorRect { x: 1, y: 7, w: 45, h: 4, color: Color::RGB(255, 255, 0) },
		ColorRect { x: 27, y: 11, w: 24, h: 8, color: Color::RGB(255, 255, 0) },
		ColorRect { x: 32, y: 19, w: 16, h: 4, color: Color::RGB(255, 255, 0) }
	]
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

pub static DR_TROOPER: DrawableListRect = DrawableListRect {
	cnt: 3,
	list_rect: [
		ColorRect { x: 7, y: 0, w: 6, h: 13, color: Color::RGB(0, 255, 0) },
		ColorRect { x: 5, y: 3, w: 10, h: 4, color: Color::RGB(0, 255, 0) },
		ColorRect { x: 9, y: 9, w: 2, h: 4, color: Color::RGB(0, 0, 0) },
		ColorRect { x: 0, y: 0, w: 0, h: 0, color: Color::RGB(0, 0, 0) },
		ColorRect { x: 0, y: 0, w: 0, h: 0, color: Color::RGB(0, 0, 0) }
	]

};

pub static DR_CHUTE: DrawableListRect = DrawableListRect {
	cnt: 5,
	list_rect: [
		ColorRect { x: 4, y: 0, w: 12, h: 2, color: Color::RGB(255, 255, 255) },
		ColorRect { x: 0, y: 2, w: 20, h: 5, color: Color::RGB(255, 255, 255) },
		ColorRect { x: 7, y: 10, w: 6, h: 13, color: Color::RGB(0, 255, 0) },
		ColorRect { x: 5, y: 13, w: 10, h: 4, color: Color::RGB(0, 255, 0) },
		ColorRect { x: 9, y: 19, w: 2, h: 4, color: Color::RGB(0, 0, 0) }
	]
};

pub static DR_BOMB: DrawableRect = DrawableRect {
	w: 10, h: 10, color: Color::RGB(255, 0, 0)
};
