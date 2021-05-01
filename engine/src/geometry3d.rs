pub struct Geometry3D {
	pub indices: Vec<u16>,
	pub attributes: Vec<f32>
}

impl Geometry3D {
	pub fn new(indices: Vec<u16>, attributes: Vec<f32>) -> Self {
		Self {
			indices,
			attributes
		}
	}
	
	pub fn create_triangle() -> Self {
		let indices = vec![
			0, 1, 2
		];

		let attributes = vec![
			 0.0,  0.5, 0.0, 0.0, 0.0, 1.0,
			-0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
			 0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
		];

		Self {
			indices,
			attributes
		}
	}

	pub fn create_plane() -> Self {
		let indices = vec![
			0, 2, 3,
			0, 1, 2
		];

		let attributes = vec![
			 0.5,  0.5, 0.0, 0.0, 0.0, 1.0,
			-0.5,  0.5, 0.0, 0.0, 0.0, 1.0,
			-0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
			 0.5, -0.5, 0.0, 0.0, 0.0, 1.0
		];

		Self {
			indices,
			attributes
		}
	}

	pub fn create_box() -> Self {
		let indices = vec![
			0,  3,  2,  // top
			0,  2,  1,
			4,  6,  7,  // bottom
			4,  5,  6,
			8,  9,  10, // right
			8,  10, 11,
			12, 15, 13, // left
			13, 15, 14,
			16, 17, 18, // front
			16, 18, 19,
			20, 23, 22, // back
			20, 22, 21
		];

		let attributes = vec![
			 0.5,  0.5,  0.5,  0.0,  1.0,  0.0, // top
			-0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
			-0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
			 0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
			 0.5, -0.5,  0.5,  0.0, -1.0,  0.0, // bottom
			-0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
			-0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
			 0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
			-0.5,  0.5,  0.5, -1.0,  0.0,  0.0, // right
			-0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
			-0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
			-0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
			 0.5,  0.5,  0.5,  1.0,  0.0,  0.0, // left
			 0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
			 0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
			 0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
			 0.5,  0.5,  0.5,  0.0,  0.0,  1.0, // front
			-0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
			-0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
			 0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
			 0.5,  0.5, -0.5,  0.0,  0.0, -1.0, // back
			-0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
			-0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
			 0.5, -0.5, -0.5,  0.0,  0.0, -1.0
		];

		Self {
			indices,
			attributes
		}
	}
}