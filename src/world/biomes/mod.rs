use crate::world::{Voxel, biomes::desert::Desert};
use rand_pcg::Pcg64;

pub mod desert;

#[derive(Clone)]
pub enum BiomeType {
    Desert(Desert),
}

impl BiomeType {
    pub fn surface_height(&self, x: i32, z: i32) -> i32 {
        match self {
            BiomeType::Desert(b) => b.surface_height(x, z),
        }
    }

    pub fn decorate(&self, voxels: &mut [Voxel], x: usize, z: usize, h: usize, rng: &mut Pcg64) {
        match self {
            BiomeType::Desert(b) => b.decorate(voxels, x, z, h, rng),
        }
    }
}
