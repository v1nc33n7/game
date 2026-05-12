use cgmath::Point3;
use cgmath::Vector3;

pub const ALL_FACES: [Face; 6] = [
    Face::Front,
    Face::Back,
    Face::Left,
    Face::Right,
    Face::Top,
    Face::Bottom,
];

pub enum Face {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl Face {
    pub fn normal(&self) -> Vector3<i32> {
        match self {
            Face::Front => Vector3::new(0, 0, 1),
            Face::Back => Vector3::new(0, 0, -1),
            Face::Left => Vector3::new(-1, 0, 0),
            Face::Right => Vector3::new(1, 0, 0),
            Face::Top => Vector3::new(0, 1, 0),
            Face::Bottom => Vector3::new(0, -1, 0),
        }
    }

    pub fn tint(&self) -> f32 {
        match self {
            Face::Top => 1.0,
            Face::Bottom => 0.4,
            Face::Front | Face::Back => 0.8,
            Face::Left | Face::Right => 0.6,
        }
    }

    pub fn vertices(&self) -> [Point3<f32>; 4] {
        match self {
            Face::Front => [
                Point3::new(0.0, 0.0, 1.0),
                Point3::new(1.0, 0.0, 1.0),
                Point3::new(1.0, 1.0, 1.0),
                Point3::new(0.0, 1.0, 1.0),
            ],
            Face::Back => [
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
            ],
            Face::Left => [
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 1.0),
                Point3::new(0.0, 1.0, 1.0),
                Point3::new(0.0, 1.0, 0.0),
            ],
            Face::Right => [
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(1.0, 1.0, 1.0),
                Point3::new(1.0, 0.0, 1.0),
            ],
            Face::Top => [
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(0.0, 1.0, 1.0),
                Point3::new(1.0, 1.0, 1.0),
                Point3::new(1.0, 1.0, 0.0),
            ],
            Face::Bottom => [
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 1.0),
                Point3::new(0.0, 0.0, 1.0),
            ],
        }
    }
}
