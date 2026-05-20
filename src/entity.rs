use cgmath::{Point3, Vector3};

use crate::{physics::PhysicsSystem, world::World};

pub trait Entity {
    fn id(&self) -> usize;
    fn get_transform(&self) -> [[f32; 4]; 4];
    fn position(&self) -> Point3<f32>;
    fn update(&mut self, dt: f32, world: &World, physics: &PhysicsSystem);
    fn apply_velocity(&mut self, velocity: Vector3<f32>, move_up: bool);
    fn model_id(&self) -> &'static str;
}
