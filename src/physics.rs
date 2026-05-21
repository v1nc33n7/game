use crate::world::{Voxel, World};
use cgmath::{Point3, Vector3};

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
            if delta.x > 0.0 {
                let right_edge = position.x + size.x / 2.0;
                position.x -= right_edge - right_edge.floor() + 0.0001;
            } else if delta.x < 0.0 {
                let left_edge = position.x - size.x / 2.0;
                position.x += (1.0 - (left_edge - left_edge.floor())) + 0.0001;
            }
            velocity.x = 0.0;
        }

        *on_ground = false;
        position.y += delta.y;
        if Self::is_colliding(world, position, size) {
            if delta.y > 0.0 {
                let head_pos = position.y + size.y;
                position.y -= head_pos - head_pos.floor() + 0.0001;
            } else if delta.y < 0.0 {
                position.y = position.y.floor() + 1.0;
                *on_ground = true;
            }
            velocity.y = 0.0;
        }

        position.z += delta.z;
        if Self::is_colliding(world, position, size) {
            if delta.z > 0.0 {
                let front_edge = position.z + size.z / 2.0;
                position.z -= front_edge - front_edge.floor() + 0.0001;
            } else if delta.z < 0.0 {
                let back_edge = position.z - size.z / 2.0;
                position.z += (1.0 - (back_edge - back_edge.floor())) + 0.0001;
            }
            velocity.z = 0.0;
        }
    }

    fn is_colliding(world: &World, position: &Point3<f32>, size: &Vector3<f32>) -> bool {
        let min_x = (position.x - size.x / 2.0 + 0.001).floor() as i32;
        let max_x = (position.x + size.x / 2.0 - 0.001).floor() as i32;
        let min_y = (position.y + 0.001).floor() as i32;
        let max_y = (position.y + size.y - 0.001).floor() as i32;
        let min_z = (position.z - size.z / 2.0 + 0.001).floor() as i32;
        let max_z = (position.z + size.z / 2.0 - 0.001).floor() as i32;

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

