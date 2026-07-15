use cgmath::Point3;

pub struct EnemyAi {
    pub target_position: Point3<f32>,
    pub speed: f32,
}

impl EnemyAi {
    pub fn new(target_position: Point3<f32>, speed: f32) -> Self {
        Self {
            target_position,
            speed,
        }
    }
}
