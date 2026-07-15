use crate::renderer::pipelines::builder::*;
use crate::renderer::*;

const SHADER: &str = include_str!("../../assets/shaders/entity.wgsl");

pub struct EntityPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl EntityPipeline {
    pub fn new(ctx: &GpuContext, camera_bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        Self {
            pipeline: PipelineBuilder::new(ctx, SHADER)
                .label("Entity Pipeline")
                .vertex_buffers(&[Vertex::desc(), InstanceRaw::desc()])
                .bind_group_layouts(&[Some(camera_bind_group_layout)])
                .build(),
        }
    }

    pub fn draw<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        camera_bind_group: &'a wgpu::BindGroup,
        model_assets: &'a HashMap<&'static str, MeshBuffer>,
        entity_instance_buffers: &'a HashMap<usize, wgpu::Buffer>,
        render_queue: &[(&'static str, usize)],
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, camera_bind_group, &[]);

        let mut currently_bound_model = "";

        for &(model_id, entity_id) in render_queue {
            if model_id != currently_bound_model
                && let Some(mesh) = model_assets.get(model_id)
            {
                pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                currently_bound_model = model_id;
            }

            if let Some(instance_buffer) = entity_instance_buffers.get(&entity_id) {
                pass.set_vertex_buffer(1, instance_buffer.slice(..));

                if let Some(mesh) = model_assets.get(model_id) {
                    pass.draw_indexed(0..mesh.index_count, 0, 0..1);
                }
            }
        }
    }
}
