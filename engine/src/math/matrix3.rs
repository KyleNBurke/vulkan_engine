use super::{Vector2, ApproxEq};
use std::ops::{Mul, MulAssign};

const IDENTITY: [[f32; 3]; 3] = [
	[1.0, 0.0, 0.0],
	[0.0, 1.0, 0.0],
	[0.0, 0.0, 1.0]
];

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Matrix3 {
	pub elements: [[f32; 3]; 3]
}

impl Matrix3 {
	pub fn new() -> Self {
		Self { elements: IDENTITY }
	}

	pub fn from(elements: [[f32; 3]; 3]) -> Self {
		Self { elements }
	}

	pub fn set(&mut self, elements: [[f32; 3]; 3]) {
		self.elements = elements;
	}

	pub fn to_padded_array(&self) -> [[f32; 4]; 3] {
		let e = &self.elements;

		[
			[e[0][0], e[0][1], e[0][2], 0.0],
			[e[1][0], e[1][1], e[1][2], 0.0],
			[e[2][0], e[2][1], e[2][2], 0.0],
		]
	}

	pub fn compose(&mut self, position: &Vector2, rotation: f32, scale: &Vector2) {
		self.elements = [
			[rotation.cos() * scale.x, -rotation.sin(), position.x],
			[rotation.sin(), rotation.cos() * scale.y, position.y],
			[0.0, 0.0, 1.0]
		];
	}

	fn base_mul(&mut self, other: &Self) {
		let a = &mut self.elements;
		let b = &other.elements;
		
		let c00 = a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0];
		let c01 = a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1];
		let c02 = a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2];

		let c10 = a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0];
		let c11 = a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1];
		let c12 = a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2];

		let c20 = a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0];
		let c21 = a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1];
		let c22 = a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2];

		a[0][0] = c00; a[0][1] = c01; a[0][2] = c02;
		a[1][0] = c10; a[1][1] = c11; a[1][2] = c12;
		a[2][0] = c20; a[2][1] = c21; a[2][2] = c22;
	}
}

impl Mul for &Matrix3 {
	type Output = Matrix3;

	fn mul(self, other: Self) -> Matrix3 {
		let mut result = Matrix3::from(self.elements);
		result.base_mul(other);

		result
	}
}

impl MulAssign<&Matrix3> for Matrix3 {
	fn mul_assign(&mut self, other: &Matrix3) {
		self.base_mul(other);
	}
}

impl ApproxEq for Matrix3 {
	fn approx_eq(&self, other: &Self, tol: f32) -> bool {
		for i in 0..3 {
			for j in 0..3 {
				if (self.elements[i][j] - other.elements[i][j]).abs() > tol {
					return false;
				}
			}
		}

		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::math::assert_approx_eq;

	#[test]
	fn new() {
		assert_eq!(Matrix3::new().elements, IDENTITY);
	}

	#[test]
	fn from() {
		let elements = [
			[0.0, 0.1, 0.2],
			[1.0, 1.1, 1.2],
			[2.0, 2.1, 2.2]
		];

		let m = Matrix3::from(elements);
		assert_eq!(m.elements, elements);
	}

	#[test]
	fn set() {
		let elements = [
			[1.0, 2.0, 3.0],
			[4.0, 5.0, 6.0],
			[7.0, 8.0, 9.0]
		];

		let mut m = Matrix3::new();
		m.set(elements);

		assert_eq!(m.elements, elements);
	}

	#[test]
	fn to_padded_array() {
		let m = Matrix3::from([
			[1.0, 2.0, 3.0],
			[4.0, 5.0, 6.0],
			[7.0, 8.0, 9.0]
		]);

		let expected = [
			[1.0, 2.0, 3.0, 0.0],
			[4.0, 5.0, 6.0, 0.0],
			[7.0, 8.0, 9.0, 0.0]
		];

		assert_eq!(m.to_padded_array(), expected);
	}

	#[test]
	fn compose() {
		let pos = Vector2::from(100.0, 200.0);
		let rot = std::f32::consts::PI;
		let scale = Vector2::from(3.0, 4.0);
		let mut m = Matrix3::new();
		m.compose(&pos, rot, &scale);

		let expected = Matrix3::from([
			[-3.0, 0.0, 100.0],
			[0.0, -4.0, 200.0],
			[0.0, 0.0, 1.0]
		]);

		assert_approx_eq(&m, &expected, 1e-6);
	}

	#[test]
	fn base_mul() {
		let mut a = Matrix3::from([
			[1.0, 2.0, 3.0],
			[4.0, 5.0, 6.0],
			[7.0, 8.0, 9.0]
		]);

		let b = Matrix3::from([
			[1.0, 2.0, 3.0],
			[4.0, 5.0, 6.0],
			[7.0, 8.0, 9.0]
		]);

		let expected = [
			[30.0, 36.0, 42.0],
			[66.0, 81.0, 96.0],
			[102.0, 126.0, 150.0]
		];

		a.base_mul(&b);
		assert_eq!(a.elements, expected);
	}

	#[test]
	fn mul() {
		let a = Matrix3::from([
			[1.0, 2.0, 3.0],
			[4.0, 5.0, 6.0],
			[7.0, 8.0, 9.0]
		]);

		let b = Matrix3::from([
			[1.0, 2.0, 3.0],
			[4.0, 5.0, 6.0],
			[7.0, 8.0, 9.0]
		]);

		let expected = [
			[30.0, 36.0, 42.0],
			[66.0, 81.0, 96.0],
			[102.0, 126.0, 150.0]
		];

		assert_eq!((&a * &b).elements, expected);
	}

	#[test]
	fn mul_assign() {
		let mut a = Matrix3::from([
			[1.0, 2.0, 3.0],
			[4.0, 5.0, 6.0],
			[7.0, 8.0, 9.0]
		]);

		let b = Matrix3::from([
			[1.0, 2.0, 3.0],
			[4.0, 5.0, 6.0],
			[7.0, 8.0, 9.0]
		]);

		let expected = [
			[30.0, 36.0, 42.0],
			[66.0, 81.0, 96.0],
			[102.0, 126.0, 150.0]
		];

		a *= &b;
		assert_eq!(a.elements, expected);
	}

	#[test]
	fn approx_eq() {
		let elements = [
			[0.0, 0.1, 0.2],
			[1.0, 1.1, 1.2],
			[2.0, 2.1, 2.2]
		];

		let a = Matrix3::from(elements);
		let b = Matrix3::from(elements);

		assert_approx_eq(&a, &b, 0.0);
	}
}