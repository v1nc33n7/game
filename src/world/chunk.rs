use cgmath::Point3;

use crate::world::voxel::*;

pub struct ChunkNeighbors<'a> {
    pub center: &'a Chunk,
    pub north: Option<&'a Chunk>,
    pub south: Option<&'a Chunk>,
    pub east: Option<&'a Chunk>,
    pub west: Option<&'a Chunk>,
}

impl<'a> ChunkNeighbors<'a> {
    pub fn get_voxel(&self, x: i32, y: i32, z: i32) -> Voxel {
        if y < 0 || y >= Chunk::HEIGHT as i32 {
            return Voxel::Air;
        }

        let chunk = match (x, z) {
            (x, _) if x < 0 => self.west,
            (x, _) if x >= Chunk::WIDTH as i32 => self.east,
            (_, z) if z < 0 => self.south,
            (_, z) if z >= Chunk::DEPTH as i32 => self.north,
            _ => Some(self.center),
        };

        chunk.map_or(Voxel::Air, |c| {
            let local_x = x.rem_euclid(Chunk::WIDTH as i32) as usize;
            let local_z = z.rem_euclid(Chunk::DEPTH as i32) as usize;

            *c.get_block(Point3::new(local_x, y as usize, local_z))
                .unwrap_or(&Voxel::Air)
        })
    }
}

#[derive(Clone)]
pub struct Chunk {
    pub voxels: Vec<Voxel>,
    pub position: Point3<i32>,
}

impl Chunk {
    pub const WIDTH: usize = 32;
    pub const HEIGHT: usize = 128;
    pub const DEPTH: usize = 32;
    pub const SIZE: usize = Self::WIDTH * Self::HEIGHT * Self::DEPTH;

    pub fn new(voxels: Vec<Voxel>, position: Point3<i32>) -> Self {
        Self { voxels, position }
    }

    pub fn index(position: Point3<usize>) -> usize {
        position.x + (position.z * Self::WIDTH) + (position.y * Self::WIDTH * Self::DEPTH)
    }

    pub fn get_block(&self, position: Point3<usize>) -> Option<&Voxel> {
        self.voxels.get(Self::index(position))
    }
}
