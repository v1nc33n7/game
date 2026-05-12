use crate::world::{Chunk, Voxel};
use cgmath::Point3;
use noise::NoiseFn;
use noise::{Fbm, Perlin};
use rand::RngExt;
use rand_pcg::Pcg64;

#[derive(Clone)]
pub struct Desert {
    noise: Fbm<Perlin>,
}

impl Desert {
    pub fn new(seed: u32) -> Self {
        let mut noise = Fbm::<Perlin>::new(seed);
        noise.octaves = 3;
        noise.frequency = 0.005;
        noise.persistence = 0.5;

        Self { noise }
    }

    pub fn surface_height(&self, x: i32, z: i32) -> i32 {
        let n = self.noise.get([x as f64, z as f64]);
        let normalized = (n + 1.0) / 2.0;
        (normalized * (Chunk::HEIGHT as f64 * 0.5)) as i32
    }

    pub fn decorate(&self, voxels: &mut [Voxel], x: usize, z: usize, h: usize, rng: &mut Pcg64) {
        self.add_dead_tries(voxels, x, z, h, rng);
        self.add_cactus(voxels, x, z, h, rng);
    }

    fn add_cactus(&self, voxels: &mut [Voxel], x: usize, z: usize, h: usize, rng: &mut Pcg64) {
        if !rng.random_bool(0.0008) {
            return;
        }

        let cactus_h = rng.random_range(2..5);

        if h + cactus_h + 1 >= Chunk::HEIGHT {
            return;
        }

        for y in 0..=cactus_h {
            let idx = Chunk::index(Point3::new(x, h + y, z));
            voxels[idx] = Voxel::Cactus;
        }

        if cactus_h > 2 && rng.random_bool(0.5) {
            let arm_y = h + rng.random_range(2..cactus_h);
            let dx = if rng.random_bool(0.5) { 1 } else { -1 };

            let arm_x = x as i32 + dx;
            if arm_x >= 0 && arm_x < Chunk::WIDTH as i32 {
                let arm_idx = Chunk::index(Point3::new(arm_x as usize, arm_y, z));
                voxels[arm_idx] = Voxel::Cactus;
                if arm_y + 1 < Chunk::HEIGHT {
                    let up_idx = Chunk::index(Point3::new(arm_x as usize, arm_y + 1, z));
                    voxels[up_idx] = Voxel::Cactus;
                }
            }
        }
    }

    fn add_dead_tries(&self, voxels: &mut [Voxel], x: usize, z: usize, h: usize, rng: &mut Pcg64) {
        if !rng.random_bool(0.0004) {
            return;
        }

        let trunk_h = rng.random_range(4..7);

        if h + trunk_h + 2 >= Chunk::HEIGHT {
            return;
        }

        for y in 0..=trunk_h {
            voxels[Chunk::index(Point3::new(x, h + y, z))] = Voxel::DeadWood;
        }

        let num_branches = rng.random_range(2..4);
        for _ in 0..num_branches {
            let branch_y = h + rng.random_range((trunk_h / 2)..trunk_h);
            let dx = rng.random_range(-1..=1);
            let dz = rng.random_range(-1..=1);

            if dx == 0 && dz == 0 {
                continue;
            }

            let mut curr_lx = x as i32;
            let mut curr_lz = z as i32;
            let branch_len = rng.random_range(1..3);

            for i in 1..=branch_len {
                curr_lx += dx;
                curr_lz += dz;
                let curr_y = branch_y + i as usize;

                if curr_lx >= 0
                    && curr_lx < Chunk::WIDTH as i32
                    && curr_lz >= 0
                    && curr_lz < Chunk::DEPTH as i32
                    && curr_y < Chunk::HEIGHT
                {
                    let idx = Chunk::index(Point3::new(curr_lx as usize, curr_y, curr_lz as usize));
                    voxels[idx] = Voxel::DeadWood;
                }
            }
        }
    }
}
