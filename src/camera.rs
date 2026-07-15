use cgmath::{Matrix4, Point3, Rad, Vector3, perspective};

#[rustfmt::skip]
const WGPU_CORRECTION: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Clone)]
pub struct Camera {
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov_y: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
    pub follow_speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            target: Point3::new(0.5, 50.0, 0.5),
            up: Vector3::unit_y(),
            fov_y: std::f32::consts::FRAC_PI_4,
            aspect: 1.0,
            near: 0.1,
            far: 1000.0,
            distance: 10.0,
            yaw: -std::f32::consts::PI,
            pitch: 0.5,
            sensitivity: 0.003,
            follow_speed: 12.0,
        }
    }
}

impl Camera {
    pub fn build_view(&self) -> [[f32; 4]; 4] {
        let eye = self.get_eye_position();

        let view = Matrix4::look_at_rh(eye, self.target, self.up);
        let perpect = perspective(Rad(self.fov_y), self.aspect, self.near, self.far);

        let corrected = WGPU_CORRECTION * perpect * view;
        corrected.into()
    }

    fn get_eye_position(&self) -> Point3<f32> {
        let offset = Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ) * self.distance;

        self.target - offset
    }

    pub fn handle_mouse(&mut self, dx: f64, dy: f64) {
        self.yaw += (dx as f32) * self.sensitivity;
        self.pitch -= (dy as f32) * self.sensitivity;

        let limit = std::f32::consts::FRAC_PI_2 - 0.01;

        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn follow(&mut self, position: Point3<f32>, dt: f32) {
        self.target.x = position.x;
        self.target.z = position.z;

        let y_diff = position.y - self.target.y;
        if y_diff.abs() > 10.0 {
            self.target.y = position.y;
        } else {
            let smoothing = 1.0 - (-self.follow_speed * dt).exp();
            self.target.y += y_diff * smoothing;
        }
    }
}
