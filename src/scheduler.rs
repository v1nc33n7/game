use std::sync::Arc;
use std::sync::mpsc::Sender;

use crate::events::EngineEvent;
use crate::renderer;
use crate::world::{Chunk, ChunkNeighbors, Generator};

pub struct TaskScheduler {
    event_bus: Sender<EngineEvent>,
}

impl TaskScheduler {
    pub fn new(event_bus: Sender<EngineEvent>) -> Self {
        Self { event_bus }
    }

    pub fn handle_event(&self, event: EngineEvent) {
        match event {
            EngineEvent::ChunkRequested { x, z, generator } => {
                self.spawn_chunk_generation(x, z, generator);
            }
            EngineEvent::MeshRequested(chunk, neighbors) => {
                self.spawn_mesh_generation(chunk, neighbors);
            }
            _ => {}
        }
    }

    fn spawn_chunk_generation(&self, x: i32, z: i32, generator: Arc<Generator>) {
        let bus = self.event_bus.clone();

        rayon::spawn(move || {
            let chunk = generator.generate_chunk(x, z);
            let _ = bus.send(EngineEvent::ChunkGenerated(chunk));
        });
    }

    fn spawn_mesh_generation(&self, chunk: Chunk, neighbors: [Option<Chunk>; 4]) {
        let bus = self.event_bus.clone();

        rayon::spawn(move || {
            let [north, south, east, west] = neighbors;

            let neighbor_refs = ChunkNeighbors {
                center: &chunk,
                north: north.as_ref(),
                south: south.as_ref(),
                east: east.as_ref(),
                west: west.as_ref(),
            };

            let (vertices, indices) = renderer::generate_chunk_mesh(&neighbor_refs);
            let _ = bus.send(EngineEvent::MeshGenerated {
                x: chunk.position.x,
                z: chunk.position.z,
                vertices,
                indices,
            });
        });
    }
}
