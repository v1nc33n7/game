use crate::events::EngineEvent;
use crate::physics::PhysicsSystem;
use crate::player::Player;
use crate::player::PlayerInput;
use crate::scheduler::TaskScheduler;
use std::sync::mpsc;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use instant::Instant;
use winit::event::{ElementState, MouseButton};
use winit::event_loop::ActiveEventLoop;
use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::camera::*;
use crate::renderer::*;
use crate::world::World;

pub struct App {
    event_sender: Sender<EngineEvent>,
    event_receiver: Receiver<EngineEvent>,
    task_scheduler: TaskScheduler,
    world: World,
    renderer: Option<Renderer>,
    camera: Camera,
    player: Player,
    player_input: PlayerInput,
    physics_system: PhysicsSystem,
    last_frame: Instant,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        App {
            event_sender: tx.clone(),
            event_receiver: rx,
            world: World::new(0),
            task_scheduler: TaskScheduler::new(tx),
            renderer: None,
            camera: Camera::default(),
            player: Player::default(),
            player_input: PlayerInput::default(),
            physics_system: PhysicsSystem::default(),
            last_frame: Instant::now(),
        }
    }

    fn process_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                EngineEvent::ChunkGenerated(chunk) => {
                    self.world.handle_new_chunk(chunk, &self.event_sender);
                }
                EngineEvent::MeshGenerated { vertices, indices } => {
                    if let Some(renderer) = &mut self.renderer {
                        renderer.add_mesh(&vertices, &indices);
                    }
                }
                EngineEvent::CameraResized { width, height } => {
                    self.camera.aspect = width as f32 / height as f32;
                }
                EngineEvent::PlayerUpdateRequested {
                    input_vector,
                    dt,
                    move_up,
                } => {
                    self.player.velocity.x = input_vector.x * self.player.speed;
                    self.player.velocity.z = input_vector.z * self.player.speed;

                    if move_up && self.player.on_ground {
                        self.player.velocity.y = self.player.move_up_force;
                    }

                    self.physics_system.step_entity(
                        &self.world,
                        &mut self.player.position,
                        &mut self.player.velocity,
                        &self.player.size,
                        &mut self.player.on_ground,
                        dt,
                    );

                    self.camera.target = self.player.position;
                }
                EngineEvent::CameraRotateRequested { dx, dy } => {
                    self.camera.handle_mouse(dx, dy);
                }

                ev => self.task_scheduler.handle_event(ev),
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let renderer =
            pollster::block_on(Renderer::init(event_loop)).expect("Failed to initialize renderer");
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("The close button was pressed, stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now.duration_since(self.last_frame).as_secs_f32();
                self.last_frame = now;

                let input_vector = self.player_input.get_vector(self.camera.yaw);
                let _ = self.event_sender.send(EngineEvent::PlayerUpdateRequested {
                    input_vector,
                    dt,
                    move_up: self.player_input.space,
                });

                self.process_events();

                if let Some(renderer) = &mut self.renderer {
                    renderer.update_camera(CameraUniform::new(self.camera.build_view()));
                    self.world
                        .request_needed_chunks(self.player.position, &self.event_sender);
                    let _ = renderer.render();
                    renderer.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(r) = &mut self.renderer {
                    r.resize(size);
                }
                let _ = self.event_sender.send(EngineEvent::CameraResized {
                    width: size.width,
                    height: size.height,
                });
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let Some(r) = &self.renderer {
                    let _ = r
                        .window
                        .set_cursor_grab(winit::window::CursorGrabMode::Locked);
                    r.window.set_cursor_visible(false);
                }
            }
            ev => {
                self.player_input.handle_input(&ev);
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            let _ = self.event_sender.send(EngineEvent::CameraRotateRequested {
                dx: delta.0,
                dy: delta.1,
            });
        }
    }
}
