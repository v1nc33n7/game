use cgmath::{Point3, Vector3};

mod input;
pub use input::PlayerInput;

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
            position: Point3::new(50.0, 70.0, 50.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            size: Vector3::new(0.6, 1.8, 0.6),
            speed: 15.0,
            move_up_force: 10.0,
            on_ground: false,
        }
    }
}
