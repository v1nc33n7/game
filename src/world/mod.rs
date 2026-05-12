use cgmath::Point3;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, mpsc::Sender},
};

mod biomes;
mod chunk;
mod generator;
mod voxel;

pub use chunk::{Chunk, ChunkNeighbors};
pub use generator::Generator;
pub use voxel::Voxel;

use crate::events::EngineEvent;

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub in_flight: HashSet<(i32, i32)>,
    pub generator: Arc<Generator>,
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            chunks: HashMap::new(),
            in_flight: HashSet::new(),
            generator: Arc::new(Generator::new(seed)),
        }
    }

    pub fn request_needed_chunks(
        &mut self,
        player_pos: Point3<f32>,
        event_bus: &Sender<EngineEvent>,
    ) {
        let missing = self.missing_chunks(player_pos, 12);
        for coord in missing {
            if self.in_flight.insert(coord) {
                let _ = event_bus.send(EngineEvent::ChunkRequested {
                    x: coord.0,
                    z: coord.1,
                    generator: self.generator.clone(),
                });
            }
        }
    }

    pub fn handle_new_chunk(&mut self, chunk: Chunk, event_bus: &Sender<EngineEvent>) {
        let coord = (chunk.position.x, chunk.position.z);
        self.chunks.insert(coord, chunk.clone());
        self.in_flight.remove(&coord);

        for neighbor_coord in self.get_neighbors_coords(coord.0, coord.1) {
            if let Some((center, neighbors)) =
                self.get_chunk_and_neighbors(neighbor_coord.0, neighbor_coord.1)
            {
                let _ = event_bus.send(EngineEvent::MeshRequested(center, neighbors));
            }
        }
    }

    fn missing_chunks(&self, pos: Point3<f32>, radius: i32) -> Vec<(i32, i32)> {
        let cx = (pos.x / Chunk::WIDTH as f32).floor() as i32;
        let cz = (pos.z / Chunk::DEPTH as f32).floor() as i32;
        let mut missing = Vec::new();
        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let coord = (
                    (cx + dx) * Chunk::WIDTH as i32,
                    (cz + dz) * Chunk::DEPTH as i32,
                );
                if !self.chunks.contains_key(&coord) {
                    missing.push(coord);
                }
            }
        }
        missing
    }

    fn get_neighbors_coords(&self, world_x: i32, world_z: i32) -> [(i32, i32); 5] {
        [
            (world_x, world_z),
            (world_x, world_z + Chunk::DEPTH as i32),
            (world_x, world_z - Chunk::DEPTH as i32),
            (world_x + Chunk::WIDTH as i32, world_z),
            (world_x - Chunk::WIDTH as i32, world_z),
        ]
    }

    fn get_chunk_and_neighbors(
        &self,
        world_x: i32,
        world_z: i32,
    ) -> Option<(Chunk, [Option<Chunk>; 4])> {
        let center = self.chunks.get(&(world_x, world_z))?;

        let n = self.chunks.get(&(world_x, world_z + Chunk::DEPTH as i32));
        let s = self.chunks.get(&(world_x, world_z - Chunk::DEPTH as i32));
        let e = self.chunks.get(&(world_x + Chunk::WIDTH as i32, world_z));
        let w = self.chunks.get(&(world_x - Chunk::WIDTH as i32, world_z));

        if n.is_some() && s.is_some() && e.is_some() && w.is_some() {
            Some((
                center.clone(),
                [n.cloned(), s.cloned(), e.cloned(), w.cloned()],
            ))
        } else {
            None
        }
    }
}
