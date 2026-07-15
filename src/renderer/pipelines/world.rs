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
                .vertex_buffers(&[Vertex::desc()])
                .bind_group_layouts(&[Some(camera_bind_group_layout)])
                .build(),
        }
    }

    pub fn draw<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        camera_bind_group: &wgpu::BindGroup,
        chunk_meshes: &HashMap<(i32, i32), MeshBuffer>,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, camera_bind_group, &[]);

        for mesh in chunk_meshes.values() {
            pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..mesh.index_count, 0, 0..1);
        }
    }
}
