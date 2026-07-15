use crate::world::Generator;
use crate::{renderer::Vertex, world::Chunk};
use std::sync::Arc;

pub enum EngineEvent {
    ChunkRequested {
        x: i32,
        z: i32,
        generator: Arc<Generator>,
    },
    ChunkGenerated(Chunk),
    MeshRequested(Chunk, [Option<Chunk>; 4]),
    MeshGenerated {
        x: i32,
        z: i32,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
}
