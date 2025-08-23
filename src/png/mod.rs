use std::fs::File;
use std::io::{ Read, Seek, SeekFrom };

use crate::zlib::decompress;

const PNG_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

pub const CHUNK_IHDR: u32 = 0x49484452;
pub const CHUNK_IDAT: u32 = 0x49444154;
pub const CHUNK_IEND: u32 = 0x49454E44; 
const MAX_BUF_SIZE: usize = 1024 * 1024;

struct PNGChunk {
	length: u32,
	ctype: u32
}

pub struct Image {
	pub width: u32,
	pub height: u32,
	pub bpp: usize,
	pub stride: usize,
	pub data: Vec<u8>
}

struct Filter {
	pos: usize
}

impl Filter {
	fn none(&mut self, src: &[u8], dst: &mut Image) {
		for x in 0..dst.stride {
			dst.data.push(src[x]);
		}
		self.pos += dst.stride;
	}

	fn sub(&mut self, src: &[u8], dst: &mut Image) {
		for x in 0..dst.bpp {
			dst.data.push(src[x]);
		}
		let base = self.pos - dst.bpp;
		for x in dst.bpp..dst.stride {
			dst.data.push((src[x] + dst.data[base + x]) as u8);
		}
		self.pos += dst.stride;
	}

	fn up(&mut self, src: &[u8], dst: &mut Image) {
		if self.pos == 0 {
			for x in 0..dst.stride {
				dst.data.push(src[x]);
			}
		} else {
			let base = self.pos - dst.stride as usize;
			for x in 0..dst.stride {
				dst.data.push(src[x] + dst.data[base + x]);
			}
		}
		self.pos += dst.stride;
	}

	fn avg(&mut self, src: &[u8], dst: &mut Image) {
		if self.pos == 0 {
			for x in 0..dst.bpp {
				dst.data.push(src[x]);
			}
			let base = self.pos - dst.bpp;
			for x in dst.bpp..dst.stride {
				dst.data.push(src[x] + (dst.data[base + x] >> 1));
			}
		} else {
			let base_up = self.pos - dst.stride;
			for x in 0..dst.bpp {
				dst.data.push(src[x] + (dst.data[base_up + x] >> 1));
			}
			let base_prev = self.pos - dst.bpp;
			for x in dst.bpp..dst.stride {
				dst.data.push(
					src[x] + ((dst.data[base_prev + x] as u16 + dst.data[base_up + x] as u16) >> 1) as u8
				);
			}
		}
		self.pos += dst.stride;
	}

	fn paeth(&mut self, src: &[u8], dst: &mut Image) {
		if self.pos == 0 {
			for x in 0..dst.bpp {
				dst.data.push(src[x]);
			}
			let base = self.pos - dst.bpp;
			for x in dst.bpp..dst.stride {
				dst.data.push(
					src[x]
					+ self.paeth_predict(dst.data[base + x], 0, 0) 
				);
			}
		} else {
			let base_up = self.pos - dst.stride;
			for x in 0..dst.bpp {
				dst.data.push(
					src[x]
					+ self.paeth_predict(0, dst.data[base_up + x], 0)
				);
			}
			let base_prev = self.pos - dst.bpp;
			let base_prev_up = base_up - dst.bpp;
			for x in dst.bpp..dst.stride {
				dst.data.push(
					src[x]
					+ self.paeth_predict(
						dst.data[base_prev + x],
						dst.data[base_up + x],
						dst.data[base_prev_up + x]
					)
				);
			}
		}
		self.pos += dst.stride;
	}

	fn paeth_predict(&self, a: u8, b: u8, c: u8) -> u8 {
		let p: i16 = a as i16 + b as i16 - c as i16;
		let pa = (p - a as i16).abs();
		let pb = (p - b as i16).abs();
		let pc = (p - c as i16).abs();
		let mut pr = c;
		if pa <= pb && pa <= pc {
			pr = a;
		} else if pb <= pc {
			pr = b;
		}
		pr
	}
}

fn get_u32(buf: &[u8]) -> u32 {
	return
		buf[3] as u32
		| ((buf[2] as u32) << 8)
		| ((buf[1] as u32) << 16)
		| ((buf[0] as u32) << 24);
}

fn read_chunk(f: &mut File, buf: &mut [u8]) -> PNGChunk {
	let slice = &mut buf[..8]; 
	f.read_exact(slice);

	PNGChunk {
		length: get_u32(slice),
		ctype: get_u32(&slice[4..])
	}
}

pub fn read_file(filename: &str) -> Image {
	let mut file = File::open(filename).unwrap();

	let mut buf: Vec<u8> = Vec::<u8>::with_capacity(MAX_BUF_SIZE);
	buf.resize(MAX_BUF_SIZE, 0);
	let mut slice = &mut buf[..8];

	file.read_exact(slice).unwrap();
	if slice != PNG_HEADER {
		panic!("PNG header not found");
	}

	let chunk = read_chunk(&mut file, &mut buf);
	if chunk.ctype != CHUNK_IHDR {
		panic!("no IHDR");
	}

	slice = &mut buf[..chunk.length as usize + 4];
	file.read_exact(slice).unwrap();

	let width = get_u32(slice);
	let height = get_u32(&slice[4..]);
	let bit_depth = slice[8];
	let color_type = slice[9];
	let compression_method = slice[10];
	let filter_method = slice[11];
	let interlace_method = slice[12];

	if bit_depth != 8 {
		panic!("bit depth {} not supported", bit_depth);
	}
	if color_type != 2 && color_type != 6 {
		panic!("color type {} not supported", color_type);
	}
	if compression_method != 0 {
		panic!("compression method {} not supported", compression_method);
	}
	if filter_method != 0 {
		panic!("filter methos {} not supported", filter_method);
	}
	if interlace_method != 0 {
		panic!("interlacing not supported");
	}

	let mut pos:usize = 0;
	loop {
		let mut slice = &mut buf[pos..];
		let chunk = read_chunk(&mut file, slice);
		if chunk.ctype == CHUNK_IEND {
			break;
		}
		if chunk.ctype == CHUNK_IDAT {
			slice = &mut buf[pos..pos + chunk.length as usize + 4];
			file.read_exact(slice);
			pos += chunk.length as usize;
		} else {
			file.seek(SeekFrom::Current(chunk.length as i64 + 4));
		}
	}
 
	let bytes = decompress(&buf[..pos]);

	let bpp = if color_type == 2 { 3 } else { 4 };
	let stride = width * bpp;
	let data_size: usize = (width * stride * height) as usize;

	let mut img = Image {
		width: width,
		height: height,
		bpp: bpp as usize,
		stride: stride as usize,
		data: Vec::<u8>::with_capacity(data_size)
	};

	println!("w: {}, h: {}", img.width, img.height);
	let mut filter = Filter {
		pos: 0
	};

	pos = 0;
	for y in 0..height { // for each scanline
		let filter_type = bytes[pos]; // first byte of scanline
		pos += 1;
		let slice = &bytes[pos..];
		if filter_type == 0 { // None
			filter.none(slice, &mut img);
		} else if filter_type == 1 { // Sub
			filter.sub(slice, &mut img);
		} else if filter_type == 2 { // Up
			filter.up(slice, &mut img);
		} else if filter_type == 3 { // Average
			filter.avg(slice, &mut img);
		} else if filter_type == 4 { // Paeth
			filter.paeth(slice, &mut img);
		} else {
			panic!("unknown filter type: {}", filter_type);
		}
		pos += img.stride;
	}
	img
}
