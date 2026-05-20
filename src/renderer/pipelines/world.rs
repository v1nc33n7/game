use crate::renderer::pipelines::builder::*;
use crate::renderer::*;

const SHADER: &str = include_str!("../../assets/shaders/shader.wgsl");

pub struct WorldPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl WorldPipeline {
    pub fn new(ctx: &GpuContext, camera_bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        Self {
            pipeline: PipelineBuilder::new(ctx, SHADER)
                .label("World Pipeline")
                .vertex_buffers(&[Vertex::layout()])
                .bind_group_layouts(&[Some(camera_bind_group_layout)])
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

    pub fn draw<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        camera_bind_group: &wgpu::BindGroup,
        chunk_meshes: &HashSet<MeshBuffer>,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, camera_bind_group, &[]);

        for mesh in chunk_meshes {
            pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..mesh.index_count, 0, 0..1);
        }
    }
}
