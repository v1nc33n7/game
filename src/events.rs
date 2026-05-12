use crate::renderer::RenderItem;
use crate::world::ChunkNeighbors;
use crate::world::Generator;
use crate::{renderer::Vertex, world::Chunk};
use std::sync::Arc;
use std::sync::mpsc::Sender;

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
}

pub struct EventScheduler {
    event_bus: Sender<EngineEvent>,
}

impl EventScheduler {
    pub fn new(event_bus: Sender<EngineEvent>) -> Self {
        Self { event_bus }
    }

    pub fn handle_event(&self, event: EngineEvent) {
        match event {
            EngineEvent::ChunkRequested { x, z, generator } => {
                let generator = Arc::clone(&generator);
                let bus = self.event_bus.clone();
                rayon::spawn(move || {
                    let chunk = generator.generate_chunk(x, z);
                    let _ = bus.send(EngineEvent::ChunkGenerated(chunk));
                });
            }
            EngineEvent::MeshRequested(chunk, neighbors_arr) => {
                let bus = self.event_bus.clone();
                rayon::spawn(move || {
                    let [north, south, east, west] = neighbors_arr;
                    let neighbors = ChunkNeighbors {
                        center: &chunk,
                        north: north.as_ref(),
                        south: south.as_ref(),
                        east: east.as_ref(),
                        west: west.as_ref(),
                    };
                    let (vertices, indices) = RenderItem::Chunk(&chunk, &neighbors).generate_mesh();
                    let _ = bus.send(EngineEvent::MeshGenerated { vertices, indices });
                });
            }
            _ => {}
        }
    }
}
