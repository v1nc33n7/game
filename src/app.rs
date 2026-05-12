use crate::events::EngineEvent;
use crate::events::EventScheduler;
use std::sync::mpsc;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use winit::event::{ElementState, MouseButton};
use winit::event_loop::ActiveEventLoop;
use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::camera::*;
use crate::renderer::*;
use crate::world::World;

pub struct App {
    event_sender: Sender<EngineEvent>,
    event_receiver: Receiver<EngineEvent>,
    event_scheduler: EventScheduler,
    world: World,
    renderer: Option<Renderer>,
    camera: Camera,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        App {
            event_sender: tx.clone(),
            event_receiver: rx,
            world: World::new(0),
            event_scheduler: EventScheduler::new(tx),
            renderer: None,
            camera: Camera::default(),
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
                ev @ EngineEvent::ChunkRequested { .. } | ev @ EngineEvent::MeshRequested(..) => {
                    self.event_scheduler.handle_event(ev);
                }
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
        let Some(renderer) = self.renderer.as_mut() else {
            log::warn!("Renderer not initialized yet");
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                log::info!("The close button was pressed, stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.camera.update();

                if let Some(renderer) = &mut self.renderer {
                    renderer.update_camera(CameraUniform::new(self.camera.build_view()));
                }

                self.world
                    .request_needed_chunks(self.camera.eye, &self.event_sender);

                self.process_events();

                if let Some(renderer) = &mut self.renderer {
                    let _ = renderer.render();
                    renderer.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                renderer.resize(size);
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                let window = &renderer.window;
                window.set_cursor_visible(false);
                let _ = window.set_cursor_grab(winit::window::CursorGrabMode::Locked);
            }
            event => {
                self.camera.handle_input(&event);
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
            self.camera.handle_mouse(delta.0, delta.1);
        }
    }
}
