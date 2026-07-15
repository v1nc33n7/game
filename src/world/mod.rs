use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use cgmath::Point3;

pub use chunk::{Chunk, ChunkNeighbors};
pub use generator::Generator;
pub use voxel::Voxel;

mod biomes;
mod chunk;
mod generator;
mod voxel;

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

    pub fn get_missing_chunks(&mut self, player_pos: Point3<f32>) -> Vec<(i32, i32)> {
        let mut requested = self.missing_chunks(player_pos, 12);
        requested.retain(|coord| !self.in_flight.contains(coord));
        requested
    }

    pub fn mark_in_flight(&mut self, coords: &[(i32, i32)]) {
        for coord in coords {
            self.in_flight.insert(*coord);
        }
    }

    pub fn handle_new_chunk(&mut self, chunk: Chunk) -> Vec<(Chunk, [Option<Chunk>; 4])> {
        let coord = (chunk.position.x, chunk.position.z);
        self.chunks.insert(coord, chunk.clone());
        self.in_flight.remove(&coord);

        let mut mesh_requests = Vec::new();
        for neighbor_coord in self.get_neighbors_coords(coord.0, coord.1) {
            if let Some((center, neighbors)) =
                self.get_chunk_and_neighbors(neighbor_coord.0, neighbor_coord.1)
            {
                mesh_requests.push((center, neighbors));
            }
        }
        mesh_requests
    }

    pub fn get_block_global(&self, x: i32, y: i32, z: i32) -> Voxel {
        if y < 0 || y >= Chunk::HEIGHT as i32 {
            return Voxel::Air;
        }

        let chunk_x = (x as f32 / Chunk::WIDTH as f32).floor() as i32 * Chunk::WIDTH as i32;
        let chunk_z = (z as f32 / Chunk::DEPTH as f32).floor() as i32 * Chunk::DEPTH as i32;

        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_z)) {
            let local_x = (x - chunk_x) as usize;
            let local_z = (z - chunk_z) as usize;

            if let Some(voxel) = chunk.get_block(Point3::new(local_x, y as usize, local_z)) {
                return *voxel;
            }
        }

        Voxel::Air
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

        if n.is_none() || s.is_none() || e.is_none() || w.is_none() {
            return None;
        }

        Some((
            center.clone(),
            [n.cloned(), s.cloned(), e.cloned(), w.cloned()],
        ))
    }
}
