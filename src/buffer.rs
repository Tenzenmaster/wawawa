use bytemuck::{Pod, Zeroable};

pub struct Buffers {
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

pub trait Vertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ColorVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl ColorVertex {
    pub const VERTEX_ARRAY: [Self; 3] = [
        ColorVertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
        ColorVertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
        ColorVertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
    ];

    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
    ];
}

impl Vertex for ColorVertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &Self::ATTRIBUTES,
    };
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TextureVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl TextureVertex {
    pub const VERTEX_ARRAY: [Self; 4] = [
        TextureVertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0] },
        TextureVertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0] },
        TextureVertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },
        TextureVertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },
    ];

    pub const INDEX_ARRAY: [u16; 6] = [
        0, 1, 2,
        0, 2, 3,
    ];

    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
    ];
}

impl Vertex for TextureVertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &Self::ATTRIBUTES,
    };
}

