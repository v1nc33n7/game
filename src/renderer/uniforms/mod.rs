use crate::renderer::context::GpuContext;
use crate::renderer::uniforms::builder::UniformBuilder;

mod builder;
mod camera;
mod entity;

pub use camera::CameraUniform;
pub use entity::EntityUniform;

pub struct UniformBindGroup {
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl UniformBindGroup {
    pub fn new(gpu_context: &GpuContext, size: u64) -> Self {
        UniformBuilder::new(gpu_context, size)
            .bind_group_layout_entries(&[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }])
            .build()
    }
}
