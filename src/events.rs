use cgmath::Vector3;

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
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    CameraResized {
        width: u32,
        height: u32,
    },
    PlayerUpdateRequested {
        input_vector: Vector3<f32>,
        dt: f32,
        move_up: bool,
    },
    CameraRotateRequested {
        dx: f64,
        dy: f64,
    },
}
