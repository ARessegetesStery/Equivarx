#[macro_export]
macro_rules! shader_src {
    ($path:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../shaders/",
            $path,
            ".wgsl"
        ))
    };
}

#[macro_export]
macro_rules! asset_file {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/", $path))
    };
}
