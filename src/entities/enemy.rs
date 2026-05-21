use cgmath::{EuclideanSpace, InnerSpace, Matrix4, Point3, Vector3};

use crate::entities::Entity;

pub struct Enemy {
    id: usize,
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub size: Vector3<f32>,
    pub speed: f32,
    pub on_ground: bool,
    pub target_position: Point3<f32>,
}

impl Enemy {
    pub fn new(id: usize, spawn_pos: Point3<f32>) -> Self {
        Self {
            id,
            position: spawn_pos,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            size: Vector3::new(0.6, 0.6, 0.6),
            speed: 4.0,
            on_ground: false,
            target_position: spawn_pos,
        }
    }
}

impl Entity for Enemy {
    fn id(&self) -> usize {
        self.id
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
        let mut direction = Vector3::new(
            self.target_position.x - self.position.x,
            0.0,
            self.target_position.z - self.position.z,
        );

        if direction.magnitude2() > 0.1 {
            direction = direction.normalize();
            self.velocity.x = direction.x * self.speed;
            self.velocity.z = direction.z * self.speed;
        } else {
            self.velocity.x = 0.0;
            self.velocity.z = 0.0;
        }

        physics.step_entity(
            world,
            &mut self.position,
            &mut self.velocity,
            &self.size,
            &mut self.on_ground,
            dt,
        );
    }

    fn apply_velocity(&mut self, velocity: Vector3<f32>, _move_up: bool) {
        self.velocity = velocity;
    }

    fn model_id(&self) -> &'static str {
        "small_cube"
    }

    fn set_target_position(&mut self, target: cgmath::Point3<f32>) {
        self.target_position = target;
    }
}
