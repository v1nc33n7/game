use cgmath::{InnerSpace, Vector3};
use winit::{event::*, keyboard::*};

#[derive(Default, Clone)]
pub struct PlayerInput {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
}

impl PlayerInput {
    pub fn get_vector(&self, yaw: f32) -> Vector3<f32> {
        let mut dir = Vector3::new(0.0, 0.0, 0.0);
        let forward = Vector3::new(yaw.cos(), 0.0, yaw.sin()).normalize();
        let right = forward.cross(Vector3::unit_y()).normalize();

        if self.w {
            dir += forward;
        }
        if self.s {
            dir -= forward;
        }
        if self.a {
            dir -= right;
        }
        if self.d {
            dir += right;
        }
        dir
    }

    pub fn handle_input(&mut self, event: &WindowEvent) {
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
                        self.w = pressed;
                    }
                    KeyCode::KeyS => {
                        self.s = pressed;
                    }
                    KeyCode::KeyA => {
                        self.a = pressed;
                    }
                    KeyCode::KeyD => {
                        self.d = pressed;
                    }
                    KeyCode::Space => {
                        self.space = pressed;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
