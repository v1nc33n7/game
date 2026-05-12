use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3, perspective};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
struct CameraInput {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    shift: bool,
    space: bool,
}

pub struct Camera {
    pub eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fov_y: f32,
    aspect: f32,
    near: f32,
    far: f32,
    input: CameraInput,
    speed: f32,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
}

#[rustfmt::skip]
const WGPU_CORRECTION: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

impl Camera {
    pub fn build_view(&self) -> [[f32; 4]; 4] {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let perpect = perspective(Rad(self.fov_y), self.aspect, self.near, self.far);
        let corrected = WGPU_CORRECTION * perpect * view;
        corrected.into()
    }

    pub fn update(&mut self) {
        let forward = Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
        let right = forward.cross(self.up).normalize();

        if self.input.w {
            self.eye += forward * self.speed;
            self.target += forward * self.speed;
        }
        if self.input.s {
            self.eye -= forward * self.speed;
            self.target -= forward * self.speed;
        }
        if self.input.a {
            self.eye -= right * self.speed;
            self.target -= right * self.speed;
        }
        if self.input.d {
            self.eye += right * self.speed;
            self.target += right * self.speed;
        }
        if self.input.shift {
            self.eye.y -= self.speed;
        }
        if self.input.space {
            self.eye.y += self.speed;
        }

        self.target = self.eye + forward;
    }

    pub fn handle_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => {
                let pressed = *state == ElementState::Pressed;
                match key {
                    KeyCode::KeyW => {
                        self.input.w = pressed;
                        true
                    }
                    KeyCode::KeyS => {
                        self.input.s = pressed;
                        true
                    }
                    KeyCode::KeyA => {
                        self.input.a = pressed;
                        true
                    }
                    KeyCode::KeyD => {
                        self.input.d = pressed;
                        true
                    }
                    KeyCode::Space => {
                        self.input.space = pressed;
                        true
                    }
                    KeyCode::ShiftLeft => {
                        self.input.shift = pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn handle_mouse(&mut self, dx: f64, dy: f64) {
        self.yaw += (dx as f32) * self.sensitivity;
        self.pitch -= (dy as f32) * self.sensitivity;

        let limit = std::f32::consts::FRAC_PI_2 - 0.01;
        self.pitch = self.pitch.clamp(-limit, limit);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Point3::new(50.0, 125.0, 30.0),
            target: Point3::new(0.5, 50.0, 0.5),
            up: Vector3::unit_y(),
            fov_y: std::f32::consts::FRAC_PI_4,
            aspect: 1.0,
            near: 0.1,
            far: 100.0,
            input: CameraInput::default(),
            speed: 2.0,
            yaw: -std::f32::consts::PI,
            pitch: 0.0,
            sensitivity: 0.005,
        }
    }
}
