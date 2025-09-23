use crate::primitives;
use crate::texture;
use anyhow::Result;
use log::info;
use std::path::{Path, PathBuf};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};
use wgpu::util::DeviceExt;

pub fn file_name_to_path(file_name: &str) -> Result<PathBuf> {
    let path = Path::new(std::env::var("OUT_DIR")?.as_str())
        .join("assets")
        .join(file_name);
    Ok(path)
}

pub async fn load_string(path: &Path) -> Result<String> {
    let data = std::fs::read_to_string(&path)?;
    Ok(data)
}

pub async fn load_binary(path: &Path) -> Result<Vec<u8>> {
    let data = std::fs::read(path)?;
    Ok(data)
}

pub async fn load_texture(
    path: &Path,
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<texture::Texture> {
    let data = load_binary(path).await?;
    texture::Texture::from_bytes(device, queue, &data, Some(file_name))
}

pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<primitives::Model> {
    let obj_path_buf = file_name_to_path(file_name)?;
    let obj_dir = obj_path_buf.parent().unwrap();
    let mut obj_reader = BufReader::new(File::open(obj_path_buf.as_path()).await?);

    let (models, obj_materials) = tobj::tokio::load_obj_buf(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            info!("{}", &p.to_str().unwrap());
            let file = File::open(obj_dir.join(p.to_str().unwrap())).await.unwrap();
            tobj::tokio::load_mtl_buf(&mut BufReader::new(file)).await
        },
    )
    .await?;

    let mut materials = Vec::new();
    for m in obj_materials? {
        let material_name = &m.diffuse_texture.unwrap();
        let diffuse_texture =
            load_texture(obj_dir.join(material_name).as_path(), material_name, device, queue).await?;
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: None,
        });

        materials.push(primitives::Material {
            name: m.name,
            diffuse_texture,
            bind_group,
        })
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| {
                    if m.mesh.normals.is_empty() {
                        primitives::ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [
                                m.mesh.texcoords[i * 2],
                                1.0 - m.mesh.texcoords[i * 2 + 1],
                            ],
                            normal: [0.0, 0.0, 0.0],
                        }
                    } else {
                        primitives::ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [
                                m.mesh.texcoords[i * 2],
                                1.0 - m.mesh.texcoords[i * 2 + 1],
                            ],
                            normal: [
                                m.mesh.normals[i * 3],
                                m.mesh.normals[i * 3 + 1],
                                m.mesh.normals[i * 3 + 2],
                            ],
                        }
                    }
                })
                .collect::<Vec<_>>();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", file_name)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            primitives::Mesh {
                name: file_name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    Ok(primitives::Model { meshes, materials })
}
