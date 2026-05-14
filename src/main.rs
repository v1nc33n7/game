use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::*;

mod app;
mod camera;
mod events;
mod physics;
mod player;
mod renderer;
mod scheduler;
mod world;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
