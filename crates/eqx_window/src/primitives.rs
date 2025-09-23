use crate::texture;
use bytemuck::{Pod, Zeroable};
use glam::{Quat, Vec3};
use std::ops::Range;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

pub trait Vertex {
    fn descriptor() -> wgpu::VertexBufferLayout<'static>;
}

impl Vertex for ModelVertex {
    fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub trait ModelRender<'a> {
    fn render_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    );
    fn render_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    );

    fn render_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup);
    fn render_model_instanced(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    );
}

impl<'a, 'b> ModelRender<'a> for wgpu::RenderPass<'b>
where
    'a: 'b,
{
    fn render_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    ) {
        self.render_mesh_instanced(mesh, material, camera_bind_group, 0..1);
    }

    fn render_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        instances: std::ops::Range<u32>,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
    
    fn render_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup) {
        self.render_model_instanced(model, camera_bind_group, 0..1);
    }
    
    fn render_model_instanced(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        instances: std::ops::Range<u32>,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.render_mesh_instanced(mesh, material, camera_bind_group, instances.clone());
        }
    }
}

pub const TEST_VERTICES: &[ModelVertex] = &[
    ModelVertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.00759614],
        normal: [0.5, 0.0, 0.5],
    }, // A
    ModelVertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
        normal: [0.5, 0.0, 0.5],
    }, // B
    ModelVertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.949397],
        normal: [0.5, 0.0, 0.5],
    }, // C
    ModelVertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.84732914],
        normal: [0.5, 0.0, 0.5],
    }, // D
    ModelVertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.2652641],
        normal: [0.5, 0.0, 0.5],
    }, // E
];

pub const TEST_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

#[derive(Clone, Copy, Debug)]
pub struct Instance {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (glam::Mat4::from_translation(self.position)
                * glam::Mat4::from_quat(self.rotation))
            .to_cols_array_2d(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub const NUM_INSTANCES_PER_ROW: u32 = 10;
pub const INSTANCE_DISPLACEMENT: Vec3 = Vec3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);
