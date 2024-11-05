use nalgebra as na;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: na::Matrix4<f32> = na::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    eye: na::Vector3<f32>,
    target: na::Vector3<f32>,
    up: na::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            eye: na::Vector3::new(0.0, 0.0, 2.0),
            target: na::Vector3::new(0.0, 0.0, 0.0),
            up: na::Vector3::new(0.0, 0.0, 0.0),
            aspect,
            fovy: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }

    pub fn matrix(&self) -> na::Matrix4<f32> {
        let view = na::Matrix4::look_at_rh(&self.eye.into(), &self.target.into(), &self.up);
        let proj = na::Matrix4::new_perspective(self.aspect, self.fovy, self.z_near, self.z_far);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        buffer: &wgpu::Buffer,
        label: Option<&str>,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
            ],
        })
    }
}
