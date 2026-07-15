use cgmath::{EuclideanSpace, Matrix4, Point3};
use std::sync::mpsc::{self, Receiver, Sender};

pub use instant::Instant;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{CursorGrabMode, WindowId};

use crate::assets::AssetManager;
use crate::ecs::{Entities, Entity};
use crate::events::EngineEvent;
use crate::input::PlayerInput;
use crate::renderer::{CameraUniform, Renderer};
use crate::scheduler::TaskScheduler;
use crate::systems::PhysicsSystem;
use crate::world::World;
use crate::{camera::*, systems};

pub struct App {
    event_sender: Sender<EngineEvent>,
    event_receiver: Receiver<EngineEvent>,
    task_scheduler: TaskScheduler,

    world: World,
    entities: Entities,
    player_id: Entity,
    physics: PhysicsSystem,

    renderer: Option<Renderer>,
    camera: Camera,
    player_input: PlayerInput,

    last_frame: Instant,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        let mut entities = Entities::default();
        let player_id = entities.spawn_player(Point3::new(50.0, 120.0, 50.0));
        entities.spawn_enemy(Point3::new(50.0, 120.0, 50.0));
        entities.spawn_enemy(Point3::new(50.0, 120.0, 50.0));

        App {
            event_sender: tx.clone(),
            event_receiver: rx,
            task_scheduler: TaskScheduler::new(tx),
            world: World::new(0),
            entities,
            player_id,
            physics: PhysicsSystem::default(),
            renderer: None,
            camera: Camera::default(),
            player_input: PlayerInput::default(),
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
                EngineEvent::MeshGenerated {
                    x,
                    z,
                    vertices,
                    indices,
                } => {
                    if let Some(renderer) = &mut self.renderer
                        && let Some(chunk_mesh) = renderer.create_vertex_buffer(&vertices, &indices)
                    {
                        renderer.chunk_meshes.insert((x, z), chunk_mesh);
                    }
                }
                ev => self.task_scheduler.handle_event(ev),
            }
        }
    }

    fn update_simulation(&mut self, dt: f32) {
        self.process_events();

        let input_vector = self.player_input.get_vector(self.camera.yaw);
        if let Some(transform) = self.entities.transforms.get_mut(&self.player_id) {
            let stats = &self.entities.player_stats[&self.player_id];
            transform.velocity.x = input_vector.x * stats.speed;
            transform.velocity.z = input_vector.z * stats.speed;
            if self.player_input.space && transform.on_ground {
                transform.velocity.y = stats.move_up_force;
            }
        }

        if let Some(transform) = self.entities.transforms.get_mut(&self.player_id) {
            self.physics.step_entity(
                &self.world,
                &mut transform.position,
                &mut transform.velocity,
                &transform.size,
                &mut transform.on_ground,
                dt,
            );
            self.camera.follow(transform.position, dt);
        }

        let player_pos = self.camera.target;
        for ai in self.entities.ai.values_mut() {
            ai.target_position = player_pos;
        }

        systems::update_enemies(&mut self.entities, &self.world, &self.physics, dt);
    }

    fn prepare_and_render(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        renderer.entity_render_queue.clear();

        for (&id, transform) in &self.entities.transforms {
            let matrix: [[f32; 4]; 4] =
                Matrix4::from_translation(transform.position.to_vec()).into();
            renderer.update_entity_transform(id.raw() as usize, matrix);

            if let Some(&model_id) = self.entities.model_ids.get(&id) {
                renderer.queue_entity_render(model_id, id.raw() as usize);
            }
        }

        let missing = self.world.get_missing_chunks(self.camera.target);
        self.world.mark_in_flight(&missing);
        for coord in missing {
            let _ = self.event_sender.send(EngineEvent::ChunkRequested {
                x: coord.0,
                z: coord.1,
                generator: self.world.generator.clone(),
            });
        }

        renderer.update_camera(CameraUniform::new(self.camera.build_view()));
        let _ = renderer.render();
        renderer.request_redraw();
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

                self.update_simulation(dt);
                self.prepare_and_render();
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
            ev => self.player_input.handle_input(&ev),
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
