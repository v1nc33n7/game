use wgpu::BindGroupLayoutEntry;

use crate::renderer::{context::GpuContext, uniforms::UniformBindGroup};

pub struct UniformBuilder<'a> {
    label: Option<&'a str>,
    gpu_context: &'a GpuContext,
    size: u64,
    bind_group_layout_entries: &'a [BindGroupLayoutEntry],
}

impl<'a> UniformBuilder<'a> {
    pub fn new(gpu_context: &'a GpuContext, size: u64) -> Self {
        Self {
            label: None,
            size,
            gpu_context,
            bind_group_layout_entries: &[],
        }
    }

    #[allow(dead_code)]
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn bind_group_layout_entries(
        mut self,
        bind_group_layout_entries: &'a [BindGroupLayoutEntry],
    ) -> Self {
        self.bind_group_layout_entries = bind_group_layout_entries;
        self
    }

    pub fn build(self) -> UniformBindGroup {
        let buffer = self
            .gpu_context
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: self.label,
                size: self.size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let bind_group_layout =
            self.gpu_context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: self.label,
                    entries: self.bind_group_layout_entries,
                });

        let bind_group = self
            .gpu_context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: self.label,
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

        UniformBindGroup {
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
