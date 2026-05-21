use cgmath::Point3;
use rayon::prelude::*;
use std::sync::mpsc::{self, Receiver, Sender};

pub use instant::Instant;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{CursorGrabMode, WindowId};

use crate::assets::AssetManager;
use crate::camera::*;
use crate::entities::{Enemy, Entity, Player, PlayerInput};
use crate::events::EngineEvent;
use crate::physics::PhysicsSystem;
use crate::renderer::{CameraUniform, Renderer};
use crate::scheduler::TaskScheduler;
use crate::world::World;

pub struct App {
    event_sender: Sender<EngineEvent>,
    event_receiver: Receiver<EngineEvent>,
    task_scheduler: TaskScheduler,

    world: World,
    player: Player,
    entities: Vec<Box<dyn Entity + Send + Sync>>,
    physics_system: PhysicsSystem,

    renderer: Option<Renderer>,
    camera: Camera,
    player_input: PlayerInput,

    last_frame: Instant,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        let entities: Vec<Box<dyn Entity + Send + Sync>> = vec![
            Box::new(Enemy::new(1, Point3::new(50.0, 120.0, 50.0))),
            Box::new(Enemy::new(2, Point3::new(60.0, 120.0, 55.0))),
        ];

        App {
            event_sender: tx.clone(),
            event_receiver: rx,
            world: World::new(0),
            task_scheduler: TaskScheduler::new(tx),
            renderer: None,
            camera: Camera::default(),
            player: Player::default(),
            entities,
            player_input: PlayerInput::default(),
            physics_system: PhysicsSystem::default(),
            last_frame: Instant::now(),
        }
    }

    fn process_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                EngineEvent::ChunkGenerated(chunk) => {
                    let mesh_requests = self.world.handle_new_chunk(chunk);
                    for (center, neighbors) in mesh_requests {
                        let _ = self
                            .event_sender
                            .send(EngineEvent::MeshRequested(center, neighbors));
                    }
                }
                EngineEvent::MeshGenerated { vertices, indices } => {
                    if let Some(renderer) = &mut self.renderer
                        && let Some(chunk_mesh) = renderer.create_vertex_buffer(&vertices, &indices)
                    {
                        renderer.chunk_meshes.insert(chunk_mesh);
                    }
                }
                ev => self.task_scheduler.handle_event(ev),
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut renderer =
            pollster::block_on(Renderer::init(event_loop)).expect("Failed to initialize renderer");

        AssetManager::load_static_assets(&mut renderer);

        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
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

                self.process_events();

                let input_vector = self.player_input.get_vector(self.camera.yaw);
                self.player
                    .apply_velocity(input_vector, self.player_input.space);
                self.player.update(dt, &self.world, &self.physics_system);

                self.camera.target = self.player.position();

                for entity in &mut self.entities {
                    entity.set_target_position(self.player.position());
                }

                self.entities.par_iter_mut().for_each(|entity| {
                    entity.update(dt, &self.world, &self.physics_system);
                });

                if let Some(renderer) = &mut self.renderer {
                    renderer.update_entity_transform(self.player.id(), self.player.get_transform());
                    renderer.queue_entity_render(self.player.model_id(), self.player.id());

                    for entity in &self.entities {
                        renderer.update_entity_transform(entity.id(), entity.get_transform());
                        renderer.queue_entity_render(entity.model_id(), entity.id());
                    }

                    renderer.update_camera(CameraUniform::new(self.camera.build_view()));

                    let missing = self.world.get_missing_chunks(self.camera.target);
                    self.world.mark_in_flight(&missing);
                    for coord in missing {
                        let _ = self.event_sender.send(EngineEvent::ChunkRequested {
                            x: coord.0,
                            z: coord.1,
                            generator: self.world.generator.clone(),
                        });
                    }

                    let _ = renderer.render();
                    renderer.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(r) = &mut self.renderer {
                    r.resize(size);
                }
                self.camera.aspect = size.width as f32 / size.height as f32;
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let Some(r) = &self.renderer {
                    let _ = r.window.set_cursor_grab(CursorGrabMode::Locked);
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
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.camera.handle_mouse(delta.0, delta.1);
        }
    }
}
