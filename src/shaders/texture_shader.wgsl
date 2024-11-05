struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct Fragment {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> Fragment {
    var fragment: Fragment;
    fragment.clip_position = vec4(vertex.position, 1.0);
    fragment.tex_coords = vertex.tex_coords;
    return fragment;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(fragment: Fragment) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, fragment.tex_coords);
}
