use crate::renderer::pipelines::builder::*;
use crate::renderer::*;

const SHADER: &str = include_str!("../../assets/shader.wgsl");

pub struct VoxelPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl VoxelPipeline {
    pub fn new(ctx: &GpuContext, camera_layout: &wgpu::BindGroupLayout) -> Self {
        Self {
            pipeline: PipelineBuilder::new(ctx, SHADER)
                .label("Voxel Pipeline")
                .vertex_buffers(&[Vertex::layout()])
                .bind_group_layouts(&[Some(camera_layout)])
                .depth_stencil(Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: Some(true),
                    depth_compare: Some(wgpu::CompareFunction::Less),
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }))
                .build(),
        }
    }
}
