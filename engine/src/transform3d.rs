use crate::math::{vector3, Vector3, Quaternion, Matrix4};

pub struct Transform3D {
	pub position: Vector3,
	pub rotation: Quaternion,
	pub scale: Vector3,
	pub matrix: Matrix4
}

impl Transform3D {
	pub fn new() -> Self {
		Self {
			position: Vector3::new(),
			rotation: Quaternion::new(),
			scale: Vector3::from_scalar(1.0),
			matrix: Matrix4::new()
		}
	}

	pub fn update_matrix(&mut self) {
		self.matrix.compose(&self.position, &self.rotation, &self.scale);
	}

	pub fn translate_on_axis(&mut self, mut axis: Vector3, distance: f32) {
		axis.apply_quaternion(&self.rotation);
		self.position += axis * distance;
	}

	pub fn translate_x(&mut self, distance: f32) {
		self.translate_on_axis(vector3::UNIT_X, distance);
	}

	pub fn translate_y(&mut self, distance: f32) {
		self.translate_on_axis(vector3::UNIT_Y, distance);
	}

	pub fn translate_z(&mut self, distance: f32) {
		self.translate_on_axis(vector3::UNIT_Z, distance);
	}

	pub fn rotate_on_axis(&mut self, axis: &Vector3, angle: f32) {
		let mut quat = Quaternion::new();
		quat.set_from_axis_angle(axis, angle);
		self.rotation *= quat;
	}

	pub fn rotate_x(&mut self, angle: f32) {
		self.rotate_on_axis(&vector3::UNIT_X, angle);
	}

	pub fn rotate_y(&mut self, angle: f32) {
		self.rotate_on_axis(&vector3::UNIT_Y, angle);
	}

	pub fn rotate_z(&mut self, angle: f32) {
		self.rotate_on_axis(&vector3::UNIT_Z, angle);
	}
}