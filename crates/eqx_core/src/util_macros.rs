#[macro_export]
macro_rules! shader_src_from {
    ($path:expr) => {{
        let path = format!(
            "{}/../../shaders/{}.wgsl",
            env!("CARGO_MANIFEST_DIR"),
            $path
        );
        std::fs::read_to_string(path).unwrap()
    }};
}

#[macro_export]
macro_rules! asset_file_path {
    ($path:expr) => {
        format!("{}/../../assets/{}", env!("CARGO_MANIFEST_DIR"), $path)
    };
}
