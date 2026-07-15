use cgmath::{Point3, Vector3};

#[derive(Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub size: Vector3<f32>,
    pub on_ground: bool,
}

impl Transform {
    pub fn new(
        position: Point3<f32>,
        velocity: Vector3<f32>,
        size: Vector3<f32>,
        on_ground: bool,
    ) -> Self {
        Self {
            position,
            velocity,
            size,
            on_ground,
        }
    }
}
