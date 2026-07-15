use crate::world::{Voxel, World};
use cgmath::{Point3, Vector3};

pub struct PhysicsSystem {
    pub gravity: f32,
    pub max_step_height: f32,
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self {
            gravity: 25.0,
            max_step_height: 1.0,
        }
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
        let was_on_ground = *on_ground;

        position.y += delta.y;
        if Self::is_colliding(world, position, size) {
            if delta.y < 0.0 {
                position.y = position.y.floor() + 1.0;
                *on_ground = true;
            } else {
                position.y = (position.y + size.y).floor() - size.y - 0.0001;
            }
            velocity.y = 0.0;
        } else {
            *on_ground = false;
        }

        let mut test_xz = *position;
        test_xz.x += delta.x;
        test_xz.z += delta.z;
        let blocked = Self::is_colliding(world, &test_xz, size);

        if blocked && was_on_ground {
            let mut step = *position;
            step.y += self.max_step_height;

            if !Self::is_colliding(world, &step, size) {
                step.x += delta.x;
                step.z += delta.z;

                if !Self::is_colliding(world, &step, size) {
                    let mut y = step.y;
                    while y >= position.y {
                        step.y = y;
                        if Self::is_colliding(world, &step, size) {
                            *position = Point3::new(step.x, y.floor() + 1.0, step.z);
                            *on_ground = true;
                            velocity.y = 0.0;
                            return;
                        }
                        y -= 0.02;
                    }
                }
            }
        }

        position.x += delta.x;
        if Self::is_colliding(world, position, size) {
            position.x = Self::resolve_axis(position.x, size.x, delta.x);
            velocity.x = 0.0;
        }

        position.z += delta.z;
        if Self::is_colliding(world, position, size) {
            position.z = Self::resolve_axis(position.z, size.z, delta.z);
            velocity.z = 0.0;
        }
    }

    fn resolve_axis(center: f32, extent: f32, delta: f32) -> f32 {
        if delta > 0.0 {
            (center + extent / 2.0).floor() - extent / 2.0 - 0.001
        } else {
            (center - extent / 2.0).floor() + 1.0 + extent / 2.0 + 0.001
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
