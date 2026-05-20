use cgmath::Point3;
use rand::SeedableRng;
use rand_pcg::Pcg64;

use crate::world::biomes::BiomeType;
use crate::world::biomes::desert::Desert;
use crate::world::{Chunk, Voxel};

pub struct Generator {
    seed: u32,
}

impl Generator {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
        let mut voxels = vec![Voxel::Air; Chunk::SIZE];
        let biome = self.get_biome(chunk_x, chunk_z);

        for x in 0..Chunk::WIDTH {
            for z in 0..Chunk::DEPTH {
                let world_x = chunk_x + x as i32;
                let world_z = chunk_z + z as i32;

                let h = biome.surface_height(world_x, world_z);
                let h_clamped = h.clamp(0, (Chunk::HEIGHT - 1) as i32) as usize;

                for y in 0..h_clamped {
                    voxels[Chunk::index(Point3::new(x, y, z))] = Voxel::Sandstone;
                }

                let mut rng = self.get_rng_at(world_x, world_z);
                biome.decorate(&mut voxels, x, z, h_clamped, &mut rng);
            }
        }

        Chunk::new(voxels, Point3::new(chunk_x, 0, chunk_z))
    }

    fn get_biome(&self, _x: i32, _z: i32) -> BiomeType {
        BiomeType::Desert(Desert::new(self.seed))
    }

    fn get_rng_at(&self, world_x: i32, world_z: i32) -> Pcg64 {
        let local_hash =
            (self.seed as u64) ^ ((world_x as u64).wrapping_shl(32)) ^ (world_z as u64);

        Pcg64::seed_from_u64(local_hash)
    }
}

