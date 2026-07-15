use cgmath::{InnerSpace, Vector3};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{ecs::Entities, systems::physics::PhysicsSystem, world::World};

pub fn update_enemies(entities: &mut Entities, world: &World, physics: &PhysicsSystem, dt: f32) {
    let ai = &entities.ai;

    entities.transforms.par_iter_mut().for_each(|(id, t)| {
        let Some(a) = ai.get(id) else { return };

        let mut dir = Vector3::new(
            a.target_position.x - t.position.x,
            0.0,
            a.target_position.z - t.position.z,
        );
        if dir.magnitude2() > 0.1 {
            dir = dir.normalize();
            t.velocity.x = dir.x * a.speed;
            t.velocity.z = dir.z * a.speed;
        } else {
            t.velocity.x = 0.0;
            t.velocity.z = 0.0;
        }

        physics.step_entity(
            world,
            &mut t.position,
            &mut t.velocity,
            &t.size,
            &mut t.on_ground,
            dt,
        );
    });
}
