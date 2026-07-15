use cgmath::{Point3, Vector3, Zero};

use crate::{
    components::{EnemyAi, PlayerStats, Transform},
    ecs::{Entities, Entity},
};

pub fn spawn_player(entities: &mut Entities, position: Point3<f32>) -> Entity {
    let id = entities.spawn();
    entities.transforms.insert(
        id,
        Transform::new(
            position,
            Vector3::zero(),
            Vector3::new(1.0, 2.0, 1.0),
            false,
        ),
    );
    entities
        .player_stats
        .insert(id, PlayerStats::new(15.0, 10.0));
    entities.model_ids.insert(id, "cube");
    id
}

pub fn spawn_enemy(entities: &mut Entities, position: Point3<f32>) -> Entity {
    let id = entities.spawn();
    entities.transforms.insert(
        id,
        Transform::new(
            position,
            Vector3::zero(),
            Vector3::new(0.6, 0.6, 0.6),
            false,
        ),
    );
    entities.ai.insert(id, EnemyAi::new(position, 4.0));
    entities.model_ids.insert(id, "small_cube");
    id
}
