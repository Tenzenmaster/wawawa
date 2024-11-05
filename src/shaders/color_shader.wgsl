struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct Fragment {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> Fragment {
    var fragment: Fragment;
    fragment.clip_position = vec4(vertex.position, 1.0);
    fragment.color = vertex.color;
    return fragment;
}

@fragment
fn fs_main(fragment: Fragment) -> @location(0) vec4<f32> {
    return vec4(fragment.color, 1.0);
}
