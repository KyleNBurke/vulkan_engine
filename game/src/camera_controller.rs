use std::time::Duration;
use engine::{Camera, glfw, math::{vector3, Euler, Order}};

const TRANSLATION_SPEED: f32 = 2.5;
const ROTATION_SPEED: f32 = 0.003;
const MAX_VERTICAL_ROTATION_ANGLE: f32 = 1.57;

pub struct CameraController {
	prev_mouse_pos_x: f32,
	prev_mouse_pos_y: f32,
	euler: Euler
}

impl CameraController {
	pub fn new(window: &glfw::Window) -> Self {
		let (mouse_pos_x, mouse_pos_y) = window.get_cursor_pos();

		Self {
			prev_mouse_pos_x: mouse_pos_x as f32,
			prev_mouse_pos_y: mouse_pos_y as f32,
			euler: Euler::new(0.0, 0.0, 0.0, Order::Yxz)
		}
	}

	pub fn poll_mouse_pos(&mut self, window: &glfw::Window) {
		let (mouse_pos_x, mouse_pos_y) = window.get_cursor_pos();

		self.prev_mouse_pos_x = mouse_pos_x as f32;
		self.prev_mouse_pos_y = mouse_pos_y as f32;
	}

	pub fn update(&mut self, window: &glfw::Window, camera: &mut Camera, delta_time: &Duration) {
		let mut translation_direction = vector3::ZERO;

		if window.get_key(glfw::Key::W) == glfw::Action::Press {
			translation_direction.z = 1.0;
		}

		if window.get_key(glfw::Key::S) == glfw::Action::Press {
			translation_direction.z = -1.0;
		}

		if window.get_key(glfw::Key::A) == glfw::Action::Press {
			translation_direction.x = 1.0;
		}

		if window.get_key(glfw::Key::D) == glfw::Action::Press {
			translation_direction.x = -1.0;
		}

		if window.get_key(glfw::Key::E) == glfw::Action::Press {
			translation_direction.y = 1.0;
		}

		if window.get_key(glfw::Key::Q) == glfw::Action::Press {
			translation_direction.y = -1.0;
		}

		translation_direction.normalize();

		let (mouse_pos_x, mouse_pos_y) = window.get_cursor_pos();
		let mouse_pos_x = mouse_pos_x as f32;
		let mouse_pos_y = mouse_pos_y as f32;
		let mouse_pos_diff_x = mouse_pos_x - self.prev_mouse_pos_x;
		let mouse_pos_diff_y = mouse_pos_y - self.prev_mouse_pos_y;

		let transform = &mut camera.transform;
		self.euler.set_from_quaternion(&transform.orientation);
		self.euler.y -= mouse_pos_diff_x * ROTATION_SPEED;
		self.euler.x += mouse_pos_diff_y * ROTATION_SPEED;
		self.euler.x = self.euler.x.max(-MAX_VERTICAL_ROTATION_ANGLE).min(MAX_VERTICAL_ROTATION_ANGLE);
		transform.orientation.set_from_euler(&self.euler);

		self.prev_mouse_pos_x = mouse_pos_x;
		self.prev_mouse_pos_y = mouse_pos_y;

		transform.translate_on_axis(translation_direction, TRANSLATION_SPEED * delta_time.as_secs_f32());
		camera.update();
	}
}