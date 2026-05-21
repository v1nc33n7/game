use cgmath::{EuclideanSpace, Matrix4, Point3, Vector3};

use crate::entities::Entity;

#[derive(Clone)]
pub struct Player {
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub size: Vector3<f32>,
    pub speed: f32,
    pub move_up_force: f32,
    pub on_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Point3::new(50.0, 120.0, 50.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            size: Vector3::new(1.0, 2.0, 1.0),
            speed: 15.0,
            move_up_force: 10.0,
            on_ground: false,
        }
    }
}

impl Entity for Player {
    fn id(&self) -> usize {
        0
    }

    fn get_transform(&self) -> [[f32; 4]; 4] {
        Matrix4::from_translation(self.position.to_vec()).into()
    }

    fn position(&self) -> Point3<f32> {
        self.position
    }

    fn update(
        &mut self,
        dt: f32,
        world: &crate::world::World,
        physics: &crate::physics::PhysicsSystem,
    ) {
        physics.step_entity(
            world,
            &mut self.position,
            &mut self.velocity,
            &self.size,
            &mut self.on_ground,
            dt,
        );
    }

    fn apply_velocity(&mut self, velocity: Vector3<f32>, move_up: bool) {
        self.velocity.x = velocity.x * self.speed;
        self.velocity.z = velocity.z * self.speed;

        if move_up && self.on_ground {
            self.velocity.y = self.move_up_force;
        }
    }

    fn model_id(&self) -> &'static str {
        "cube"
    }
}
