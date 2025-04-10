struct VertexInput {
    @location(0) vert_pos: vec3<f32>,
    @location(1) vert_color: vec3<f32>,
    @location(2) vert_tex: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.vert_pos, 1.0);
    out.color = model.vert_color;
    out.tex_coord = model.vert_tex;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coord);
}

