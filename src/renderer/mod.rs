use std::collections::HashSet;
use std::sync::Arc;
use wgpu::util::DeviceExt;

use wgpu::{CommandEncoder, SurfaceTexture, TextureView};
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window};

use camera_uniform::*;
use context::*;
use pipelines::*;

mod camera_uniform;
mod context;
mod mesher;
mod pipelines;
mod vertex;

pub use camera_uniform::CameraUniform;
pub use mesher::MeshBuffer;
pub use mesher::RenderItem;
pub use vertex::Vertex;

pub struct Renderer {
    gpu_context: GpuContext,
    pub window: Arc<Window>,
    pipelines: Vec<Pipeline>,
    camera_binding_group: CameraBindGroup,
    mesh_buffers: HashSet<MeshBuffer>,
}

impl Renderer {
    pub async fn init(event_loop: &ActiveEventLoop) -> anyhow::Result<Self> {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title("Game")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
        let window = Arc::new(event_loop.create_window(window_attributes)?);
        let ctx = GpuContext::new(event_loop.owned_display_handle(), Arc::clone(&window)).await?;
        let camera_binding_group = CameraBindGroup::new(&ctx);
        let pipelines = vec![Pipeline::Voxel(VoxelPipeline::new(
            &ctx,
            &camera_binding_group.layout,
        ))];

        let renderer = Self {
            gpu_context: ctx,
            window,
            pipelines,
            camera_binding_group,
            mesh_buffers: HashSet::new(),
        };

        renderer.configure_surface();
        Ok(renderer)
    }

    pub fn add_mesh(&mut self, vertices: &[Vertex], indices: &[u32]) {
        if vertices.is_empty() {
            return;
        }

        let vertex_buf =
            self.gpu_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Mesh Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let index_buf =
            self.gpu_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Mesh Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

        self.mesh_buffers
            .insert(MeshBuffer::new(vertex_buf, index_buf, indices.len() as u32));
    }

    pub fn update_camera(&self, view: CameraUniform) {
        self.gpu_context.queue.write_buffer(
            &self.camera_binding_group.buffer,
            0,
            bytemuck::bytes_of(&view),
        );
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        let surface_texture = match self.acquire_frame() {
            Some(texture) => texture,
            None => return Ok(()),
        };

        let view = surface_texture.texture.create_view(&Default::default());
        let mut encoder = self
            .gpu_context
            .device
            .create_command_encoder(&Default::default());

        self.draw_frame(&mut encoder, &view);

        self.gpu_context.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.gpu_context.config.width = new_size.width;
        self.gpu_context.config.height = new_size.height;
        self.configure_surface();
        self.gpu_context.new_depth_texture();
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn configure_surface(&self) {
        self.gpu_context
            .surface
            .configure(&self.gpu_context.device, &self.gpu_context.config);
    }

    fn acquire_frame(&mut self) -> Option<SurfaceTexture> {
        match self.gpu_context.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => Some(surface_texture),
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                drop(surface_texture);
                self.configure_surface();
                None
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => None,
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                None
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                match self
                    .gpu_context
                    .instance
                    .create_surface(Arc::clone(&self.window))
                {
                    Ok(surface) => {
                        self.gpu_context.surface = surface;
                        self.configure_surface();
                    }
                    Err(e) => {
                        log::error!("Failed to recreate surface: {:?}", e);
                        return None;
                    }
                }
                None
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                log::error!(
                    "Surface texture validation error: no error scope registered, GPU validation failed"
                );
                None
            }
        }
    }

    fn draw_frame(&mut self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.08,
                        g: 0.10,
                        b: 0.14,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.gpu_context.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        for pipeline in &self.pipelines {
            for mesh in &self.mesh_buffers {
                pipeline.draw(
                    &mut pass,
                    &mesh.vertex_buffer,
                    &mesh.index_buffer,
                    mesh.index_count,
                    &self.camera_binding_group.bind_group,
                );
            }
        }
    }
}
