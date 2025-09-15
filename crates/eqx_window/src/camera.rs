use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

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
