use utilities::Window;

use engine::{Renderer,
	Camera,
	lights::AmbientLight,
	scene::Scene,
	pool::Pool,
	math::Vector3,
	Geometry3D,
	Transform3D,
	mesh::{
		Material,
		StaticInstancedMesh,
		InstancedMesh
	}
};

fn main() {
	let mut window = Window::new("Instancing");
	let mut renderer = Renderer::new(&window.glfw, &window.glfw_window);

	let (width, height) = window.glfw_window.get_framebuffer_size();

	let mut camera = Camera::new(width as f32 / height as f32, 75.0, 0.1, 50.0);
	camera.transform.position.set(0.0, -2.0, 0.0);
	camera.transform.rotate_y(3.14 / 4.0);
	camera.transform.rotate_x(-3.14 / 6.0);

	let ambient_light = AmbientLight::from(Vector3::from_scalar(1.0), 0.01);
	let mut scene = Scene::new(camera, ambient_light);

	let mut geometries = Pool::<Geometry3D>::new();
	let static_geometry = geometries.add(Geometry3D::create_box());
	let mut static_instanced_box = StaticInstancedMesh::new(static_geometry, Material::Basic);

	for i in 0..20 {
		for j in 0..20 {
			let mut transform = Transform3D::new();
			transform.position.set((i * 2) as f32, 0.0, (j * 2) as f32);
			static_instanced_box.transforms.push(transform);
		}
	}

	renderer.submit_static_meshes(&geometries, &mut vec![], &mut vec![static_instanced_box]);

	let geometry = scene.geometries.add(Geometry3D::create_box());
	let mut instanced_box = InstancedMesh::new(geometry, Material::Basic);

	for i in 0..20 {
		for j in 0..20 {
			let mut transform = Transform3D::new();
			transform.position.set((i * 2) as f32, 1.5, (j * 2) as f32);
			instanced_box.transforms.push(transform);
		}
	}

	let instanced_box_handle = scene.instanced_meshes.add(instanced_box);

	let mut surface_changed = false;

	window.main_loop(|resized, width, height| {
		if resized || surface_changed {
			renderer.handle_resize(width, height);
			scene.camera.projection_matrix.make_perspective(width as f32 / height as f32, 75.0, 0.1, 50.0);
		}

		let instanced_box = scene.instanced_meshes.get_mut(&instanced_box_handle).unwrap();
		for transform in &mut instanced_box.transforms {
			transform.rotate_y(0.001);
			transform.update_matrix();
		}

		surface_changed = renderer.render(&mut scene);
	});
}