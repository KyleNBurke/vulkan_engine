mod field;
mod atlas;

use std::{fs, io::Write};

pub struct Glyph {
	char_code: usize,
	bearing: (f32, f32),
	advance: f32,
	field: Vec<Vec<u8>>,
	position: (usize, usize)
}

fn main() {
	let input_font_file_path = "../arial.ttf";
	let output_bitmap_file_path = "../arial_new.bmp";
	let output_fdf_file_path = "../arial.fdf";
	let size = 64;
	let spread = 4;
	
	let (mut glyphs, space_advance) = field::load_glyphs_and_generate_sdfs(input_font_file_path, size, spread);
	let atlas = atlas::generate_atlas(&mut glyphs);

	save_to_bitmap(output_bitmap_file_path, &atlas);
	save_to_font_file(output_fdf_file_path, &mut glyphs, space_advance, &atlas);
}

fn save_to_bitmap(file_path: &str, atlas: &Vec<Vec<u8>>) {
	let image_width = atlas[0].len();
	let image_height = atlas.len();
	let image_row_padding_len = (4 - image_width % 4) % 4;

	let mut buffer: Vec<u8> = Vec::with_capacity(1078 + (image_width + image_row_padding_len) * image_height);

	// Header
	buffer.push(66u8);
	buffer.push(77u8);

	let file_size = 0u32.to_ne_bytes();
	buffer.extend_from_slice(&file_size);

	let reserved = 0u16.to_ne_bytes();
	buffer.extend_from_slice(&reserved);
	buffer.extend_from_slice(&reserved);

	let pixel_data_offset = 0u32.to_ne_bytes();
	buffer.extend_from_slice(&pixel_data_offset);

	// Info header
	let header_size = 40u32.to_ne_bytes();
	buffer.extend_from_slice(&header_size);

	let image_width_i32 = (image_width as i32).to_ne_bytes();
	buffer.extend_from_slice(&image_width_i32);

	let image_height_i32 = (image_height as i32).to_ne_bytes();
	buffer.extend_from_slice(&image_height_i32);

	let planes = 1u16.to_ne_bytes();
	buffer.extend_from_slice(&planes);

	let bpp = 8u16.to_ne_bytes();
	buffer.extend_from_slice(&bpp);

	let compression_type = 0u32.to_ne_bytes();
	buffer.extend_from_slice(&compression_type);

	let compressed_image_size = 0u32.to_ne_bytes();
	buffer.extend_from_slice(&compressed_image_size);

	let x_pixels_per_meter = 0i32.to_ne_bytes();
	buffer.extend_from_slice(&x_pixels_per_meter);

	let y_pixels_per_meter = 0i32.to_ne_bytes();
	buffer.extend_from_slice(&y_pixels_per_meter);

	let total_colors = 0u32.to_ne_bytes();
	buffer.extend_from_slice(&total_colors);

	let important_colors = 0u32.to_ne_bytes();
	buffer.extend_from_slice(&important_colors);

	// Color table
	for i in 0..256 {
		let i_u8 = i as u8;
		buffer.push(i_u8);
		buffer.push(i_u8);
		buffer.push(i_u8);
		buffer.push(0u8);
	}

	// Pixel data offset in header
	let pixel_data_offset = (buffer.len() as u32).to_ne_bytes();
	for i in 0..4 { buffer[10 + i] = pixel_data_offset[i] };

	// Pixel data
	let padding = vec![0u8; image_row_padding_len];
	for row in atlas.iter().rev() {
		for texel in row {
			buffer.push(*texel);
		}

		buffer.extend_from_slice(&padding);
	}

	// File size in header
	let file_size = (buffer.len() as u32).to_ne_bytes();
	for i in 0..4 { buffer[2 + i] = file_size[i] };

	let mut file = fs::File::create(file_path).unwrap();
	file.write_all(&buffer).unwrap();
}

fn save_to_font_file(file_path: &str, glyphs: &mut Vec<Glyph>, space_advance: f32, atlas: &Vec<Vec<u8>>) {
	glyphs.sort_unstable_by_key(|g| g.char_code);

	let atlas_width = atlas[0].len();
	let atlas_height = atlas.len();
	let glyph_count = glyphs.len();

	let mut buffer: Vec<u8> = Vec::with_capacity(12 + atlas_width * atlas_height + 32 * glyph_count);
	
	let atlas_width = (atlas_width as u32).to_ne_bytes();
	buffer.extend_from_slice(&atlas_width);

	let atlas_height = (atlas_height as u32).to_ne_bytes();
	buffer.extend_from_slice(&atlas_height);

	for row in atlas {
		buffer.extend_from_slice(row);
	}

	let space_advance = space_advance.to_ne_bytes();
	buffer.extend_from_slice(&space_advance);
	
	let glyph_count = (glyph_count as u32).to_ne_bytes();
	buffer.extend_from_slice(&glyph_count);

	for glyph in glyphs {
		let char_code = (glyph.char_code as u32).to_ne_bytes();
		buffer.extend_from_slice(&char_code);

		let position_x = (glyph.position.0 as f32).to_ne_bytes();
		buffer.extend_from_slice(&position_x);

		let position_y = (glyph.position.1 as f32).to_ne_bytes();
		buffer.extend_from_slice(&position_y);

		let width = (glyph.field[0].len() as f32).to_ne_bytes();
		buffer.extend_from_slice(&width);

		let height = (glyph.field.len() as f32).to_ne_bytes();
		buffer.extend_from_slice(&height);

		let bearing_x = glyph.bearing.0.to_ne_bytes();
		buffer.extend_from_slice(&bearing_x);

		let bearing_y = (-glyph.bearing.1).to_ne_bytes();
		buffer.extend_from_slice(&bearing_y);

		let advance = glyph.advance.to_ne_bytes();
		buffer.extend_from_slice(&advance);
	}

	let mut file = fs::File::create(file_path).unwrap();
	file.write_all(&buffer).unwrap();
}