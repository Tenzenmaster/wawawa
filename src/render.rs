use crate::{
    buffer::{Buffers, ColorVertex, TextureVertex, Vertex},
    camera::Camera,
    texture::Texture,
};

use wgpu::util::DeviceExt as _;

pub struct RenderPass {
    pipeline: wgpu::RenderPipeline,
    buffers: Buffers,
    texture_bind_group: wgpu::BindGroup,
    camera: Camera,
    camera_bind_group: wgpu::BindGroup,
}

impl RenderPass {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        log::info!("Creating render pass");

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&TextureVertex::VERTEX_ARRAY),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let vertex_count = TextureVertex::VERTEX_ARRAY.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&TextureVertex::INDEX_ARRAY),
            usage: wgpu::BufferUsages::INDEX,
        });
        let index_count = TextureVertex::INDEX_ARRAY.len() as u32;

        let buffers = Buffers {
            vertex_buffer,
            vertex_count,
            index_buffer,
            index_count,
        };

        let texture_bind_group_layout = Texture::create_bind_group_layout(device);

        let texture = Texture::from_bytes(
            device,
            queue,
            include_bytes!("textures/crate.jpg"),
            Some("Cool texture"),
        )
        .expect("Failed to load crate texture");
        let texture_bind_group =
            texture.create_bind_group(device, &texture_bind_group_layout, Some("Cool texture"));

        let camera_bind_group_layout = Camera::create_bind_group_layout(device);

        let camera = Camera::new(config.width as f32 / config.height as f32);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::bytes_of(&camera.matrix()),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let camera_bind_group = camera.create_bind_group(
            device,
            &camera_bind_group_layout,
            &camera_buffer,
            Some("Camera bind group"),
        );

        let shader_module =
            device.create_shader_module(wgpu::include_wgsl!("shaders/texture_shader.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[TextureVertex::LAYOUT],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            multiview: None,
            cache: None,
        });
        log::info!("Render pass created");

        Self {
            pipeline,
            buffers,
            texture_bind_group,
            camera,
            camera_bind_group,
        }
    }

    pub fn draw(&self, device: &wgpu::Device, queue: &wgpu::Queue, surface: &wgpu::Surface) {
        let output = surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.buffers.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.buffers.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.buffers.index_count, 0, 0..1);

        drop(render_pass);

        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
