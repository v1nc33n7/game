use crate::renderer::context::GpuContext;
use wgpu::util::DeviceExt;

mod chunk;
mod face;
mod vertex;

pub use chunk::generate_chunk_mesh;
pub use vertex::Vertex;

#[derive(PartialEq, Eq, Hash)]
pub struct MeshBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl MeshBuffer {
    pub fn new<T: bytemuck::Pod>(
        gpu_context: &GpuContext,
        vertices: &[T],
        indices: &[u32],
    ) -> Self {
        Self {
            vertex_buffer: gpu_context.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Mesh Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ),
            index_buffer: gpu_context.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Mesh Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                },
            ),
            index_count: indices.len() as u32,
        }
    }
}
