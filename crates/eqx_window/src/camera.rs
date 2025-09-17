use bytemuck::{Pod, Zeroable};
use eqx_utils::input::{Input, KeyState};
use glam::{Mat4, Vec3};
use winit::keyboard::KeyCode;

pub struct Camera {
    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
}

impl Camera {
    fn view_projection_matrix(&self) -> Mat4 {
        #[rustfmt::skip]
        pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols(
            glam::Vec4::new(1.0, 0.0, 0.0, 0.0),
            glam::Vec4::new(0.0, 1.0, 0.0, 0.0),
            glam::Vec4::new(0.0, 0.0, 0.5, 0.0),
            glam::Vec4::new(0.0, 0.0, 0.5, 1.0),
        );

        let view = Mat4::look_at_rh(self.pos, self.target, self.up);
        let projection = Mat4::perspective_rh(self.fov, self.aspect, self.near_clip, self.far_clip);

        OPENGL_TO_WGPU_MATRIX * projection * view
    }
}

// Refrain from switching to encase: deriving `ShaderType` using `impl_matrix!` is very hard to use
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.view_projection_matrix().to_cols_array_2d();
    }
}

pub struct CameraController {
    pub speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn update_camera(&self, camera: &mut Camera, input: &Input) {
        let forward = camera.target - camera.pos;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        let forward_pressed = input.lookup(&KeyCode::KeyW) == KeyState::Pressed
            || input.lookup(&KeyCode::ArrowUp) == KeyState::Pressed;

        let backward_pressed = input.lookup(&KeyCode::KeyS) == KeyState::Pressed
            || input.lookup(&KeyCode::ArrowDown) == KeyState::Pressed;

        let right_pressed = input.lookup(&KeyCode::KeyD) == KeyState::Pressed
            || input.lookup(&KeyCode::ArrowRight) == KeyState::Pressed;

        let left_pressed = input.lookup(&KeyCode::KeyA) == KeyState::Pressed
            || input.lookup(&KeyCode::ArrowLeft) == KeyState::Pressed;

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if forward_pressed && forward_mag > self.speed {
            camera.pos += forward_norm * self.speed;
        }
        if backward_pressed {
            camera.pos -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.pos;
        let forward_mag = forward.length();

        if right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.pos = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if left_pressed {
            camera.pos = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
