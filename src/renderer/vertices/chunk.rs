use crate::renderer::Vertex;
use crate::renderer::vertices::face::ALL_FACES;
use crate::world::{Chunk, ChunkNeighbors, Voxel};
use cgmath::Point3;

pub fn generate_chunk_mesh(neighbors: &ChunkNeighbors) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let chunk = neighbors.center;

    for x in 0..Chunk::WIDTH {
        for y in 0..Chunk::HEIGHT {
            for z in 0..Chunk::DEPTH {
                let block = match chunk.get_block(Point3::new(x, y, z)) {
                    Some(b) => b,
                    None => continue,
                };

                if matches!(block, Voxel::Air) {
                    continue;
                }

                for face in ALL_FACES {
                    let nv = face.normal();
                    let nx = x as i32 + nv.x;
                    let ny = y as i32 + nv.y;
                    let nz = z as i32 + nv.z;

                    if matches!(neighbors.get_voxel(nx, ny, nz), Voxel::Air) {
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
    }
    (vertices, indices)
}
