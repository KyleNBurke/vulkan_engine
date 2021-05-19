use crate::{Transform3D, pool::Handle};

#[derive(Copy, Clone)]
pub enum Material {
	Basic,
	Normal,
	Lambert
}

pub struct StaticMesh {
	pub transform: Transform3D,
	pub geometry_handle: Handle,
	pub material: Material
}

impl StaticMesh {
	pub fn new(geometry_handle: Handle, material: Material) -> Self {
		Self {
			transform: Transform3D::new(),
			geometry_handle,
			material
		}
	}
}

pub struct Mesh {
	pub geometry_handle: Handle,
	pub material: Material
}

impl Mesh {
	pub fn new(geometry_handle: Handle, material: Material) -> Self {
		Self {
			geometry_handle,
			material
		}
	}
}