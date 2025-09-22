use crate::texture;
use anyhow::Result;

pub async fn load_binary(filename: &str) -> Result<Vec<u8>> {
    let data = {
        let path = std::path::Path::new(std::env::var("OUT_DIR")?.as_str())
            .join("assets")
            .join(filename);
        std::fs::read(path)?
    };
    Ok(data)
}

pub async fn load_texture(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<texture::Texture> {
    let data = load_binary(file_name).await?;
    texture::Texture::from_bytes(device, queue, &data, Some(file_name))
}
