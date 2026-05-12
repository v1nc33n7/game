mod builder;
mod voxel;

pub use voxel::VoxelPipeline;

pub enum Pipeline {
    Voxel(VoxelPipeline),
}

impl Pipeline {
    pub fn draw<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        index_count: u32,
        camera_bind_group: &'a wgpu::BindGroup,
    ) {
        let pipeline = match self {
            Pipeline::Voxel(p) => &p.pipeline,
        };
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, camera_bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..index_count, 0, 0..1);
    }
}
