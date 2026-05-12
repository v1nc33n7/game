#[derive(Clone, Copy, PartialEq)]
pub enum Voxel {
    Air,
    Sandstone,
    Cactus,
    DeadWood,
}

impl Voxel {
    pub fn color(&self) -> [f32; 3] {
        match self {
            Voxel::Air => [0.0, 0.0, 0.0],
            Voxel::Sandstone => [0.69, 0.48, 0.32],
            Voxel::Cactus => [0.35, 0.42, 0.21],
            Voxel::DeadWood => [0.60, 0.57, 0.52],
        }
    }

    pub fn color_at(&self, x: i32, y: i32, z: i32) -> [f32; 3] {
        let base = self.color();
        let offset = ((x ^ y ^ z) % 10) as f32 / 200.0;
        [
            (base[0] + offset).clamp(0.0, 1.0),
            (base[1] + offset).clamp(0.0, 1.0),
            (base[2] + offset).clamp(0.0, 1.0),
        ]
    }
}
