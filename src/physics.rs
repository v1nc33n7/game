use cgmath::{Point3, Vector3};

use crate::world::{Voxel, World};

pub struct PhysicsSystem {
    pub gravity: f32,
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self { gravity: 25.0 }
    }
}

impl PhysicsSystem {
    pub fn step_entity(
        &self,
        world: &World,
        position: &mut Point3<f32>,
        velocity: &mut Vector3<f32>,
        size: &Vector3<f32>,
        on_ground: &mut bool,
        dt: f32,
    ) {
        velocity.y -= self.gravity * dt;

        let delta = *velocity * dt;

        position.x += delta.x;
        if Self::is_colliding(world, position, size) {
            position.x -= delta.x;
            velocity.x = 0.0;
        }

        *on_ground = false;
        position.y += delta.y;
        if Self::is_colliding(world, position, size) {
            position.y -= delta.y;
            if velocity.y < 0.0 {
                *on_ground = true;
            }
            velocity.y = 0.0;
        }

        position.z += delta.z;
        if Self::is_colliding(world, position, size) {
            position.z -= delta.z;
            velocity.z = 0.0;
        }
    }

    fn is_colliding(world: &World, position: &Point3<f32>, size: &Vector3<f32>) -> bool {
        let min_x = (position.x - size.x / 2.0).floor() as i32;
        let max_x = (position.x + size.x / 2.0).floor() as i32;
        let min_y = position.y.floor() as i32;
        let max_y = (position.y + size.y).floor() as i32;
        let min_z = (position.z - size.z / 2.0).floor() as i32;
        let max_z = (position.z + size.z / 2.0).floor() as i32;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if world.get_block_global(x, y, z) != Voxel::Air {
                        return true;
                    }
                }
            }
        }
        false
    }
}
