use cgmath::{Point3, Vector3, Zero};

use crate::{
    components::{EnemyAi, PlayerStats, Transform},
    ecs::{Entities, Entity},
};

impl Entities {
    pub fn spawn_player(&mut self, position: Point3<f32>) -> Entity {
        self.spawn_builder()
            .with_transform(Transform::new(
                position,
                Vector3::zero(),
                Vector3::new(1.0, 2.0, 1.0),
                false,
            ))
            .with_model_id("cube")
            .with_player_stats(PlayerStats::new(16.0, 10.0))
            .id()
    }

    pub fn spawn_enemy(&mut self, position: Point3<f32>) -> Entity {
        self.spawn_builder()
            .with_transform(Transform::new(
                position,
                Vector3::zero(),
                Vector3::new(0.6, 0.6, 0.6),
                false,
            ))
            .with_model_id("small_cube")
            .with_ai(EnemyAi::new(position, 12.0))
            .id()
    }
}
