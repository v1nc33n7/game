use cgmath::Point3;

use crate::renderer::mesher::face::*;
use crate::renderer::*;
use crate::world::*;

mod face;

#[derive(PartialEq, Eq, Hash)]
pub struct MeshBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl MeshBuffer {
    pub fn new(vertex_buffer: wgpu::Buffer, index_buffer: wgpu::Buffer, index_count: u32) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }
}

pub enum RenderItem<'a> {
    Chunk(&'a Chunk, &'a ChunkNeighbors<'a>),
}

impl<'a> RenderItem<'a> {
    pub fn generate_mesh(&self) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        match self {
            RenderItem::Chunk(chunk, neighbors) => {
                for x in 0..Chunk::WIDTH {
                    for y in 0..Chunk::HEIGHT {
                        for z in 0..Chunk::DEPTH {
                            let block = match chunk.get_block(Point3::new(x, y, z)) {
                                Some(b) => b,
                                None => continue,
                            };

                            if matches!(block, &Voxel::Air) {
                                continue;
                            }

                            for face in ALL_FACES {
                                let nv = face.normal();

                                let nx = x as i32 + nv.x;
                                let ny = y as i32 + nv.y;
                                let nz = z as i32 + nv.z;

                                let neighbor_voxel = neighbors.get_voxel(nx, ny, nz);

                                if matches!(neighbor_voxel, Voxel::Air) {
                                    let idx = vertices.len() as u32;
                                    let tint = face.tint();
                                    let c = block.color_at(nx, ny, nz);
                                    let tinted = [c[0] * tint, c[1] * tint, c[2] * tint];

                                    for v in face.vertices() {
                                        vertices.push(Vertex::new(
                                            [
                                                x as f32 + v.x + chunk.position.x as f32,
                                                y as f32 + v.y + chunk.position.y as f32,
                                                z as f32 + v.z + chunk.position.z as f32,
                                            ],
                                            tinted,
                                        ));
                                    }

                                    indices.extend_from_slice(&[
                                        idx,
                                        idx + 1,
                                        idx + 2,
                                        idx,
                                        idx + 2,
                                        idx + 3,
                                    ]);
                                }
                            }
                        }
                    }
                }

                (vertices, indices)
            }
        }
    }
}
