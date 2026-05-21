use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use wgpu::{CommandEncoder, SurfaceTexture, TextureView};
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window};

use crate::renderer::uniforms::UniformBindGroup;
use context::*;
use pipelines::*;

mod context;
mod pipelines;
mod uniforms;
mod vertices;

pub use uniforms::{CameraUniform, EntityUniform};
pub use vertices::{MeshBuffer, Vertex, generate_chunk_mesh};

pub struct Renderer {
    pub gpu_context: GpuContext,
    pub window: Arc<Window>,

    pub entity_pipeline: EntityPipeline,
    pub world_pipeline: WorldPipeline,

    pub camera_binding_group: UniformBindGroup,

    pub chunk_meshes: HashSet<MeshBuffer>,
    pub entity_bind_groups: HashMap<usize, UniformBindGroup>,
    pub model_assets: HashMap<&'static str, MeshBuffer>,

    pub entity_render_queue: Vec<(&'static str, usize)>,
}

impl Renderer {
    pub async fn init(event_loop: &ActiveEventLoop) -> anyhow::Result<Self> {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title("Game")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
        let window = Arc::new(event_loop.create_window(window_attributes)?);
        let gpu_context =
            GpuContext::new(event_loop.owned_display_handle(), Arc::clone(&window)).await?;

        let camera_binding_group =
            UniformBindGroup::new(&gpu_context, std::mem::size_of::<CameraUniform>() as u64);
        let entity_binding_group =
            UniformBindGroup::new(&gpu_context, std::mem::size_of::<EntityUniform>() as u64);

        let world_pipeline =
            WorldPipeline::new(&gpu_context, &camera_binding_group.bind_group_layout);
        let entity_pipeline = EntityPipeline::new(
            &gpu_context,
            &camera_binding_group.bind_group_layout,
            &entity_binding_group.bind_group_layout,
        );

        let renderer = Self {
            gpu_context,
            window,
            camera_binding_group,
            chunk_meshes: HashSet::new(),
            entity_bind_groups: HashMap::new(),
            world_pipeline,
            entity_pipeline,
            model_assets: HashMap::new(),
            entity_render_queue: Vec::new(),
        };

        renderer.configure_surface();
        Ok(renderer)
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.gpu_context.config.width = new_size.width;
        self.gpu_context.config.height = new_size.height;
        self.configure_surface();
        self.gpu_context.new_depth_texture();
    }

    fn configure_surface(&self) {
        self.gpu_context
            .surface
            .configure(&self.gpu_context.device, &self.gpu_context.config);
    }

    pub fn create_vertex_buffer<T: bytemuck::Pod>(
        &self,
        vertices: &[T],
        indices: &[u32],
    ) -> Option<MeshBuffer> {
        (!vertices.is_empty() && !indices.is_empty())
            .then(|| MeshBuffer::new(&self.gpu_context, vertices, indices))
    }

    pub fn register_model_asset(
        &mut self,
        name: &'static str,
        vertices: &[Vertex],
        indices: &[u32],
    ) {
        if let Some(mesh) = self.create_vertex_buffer(vertices, indices) {
            self.model_assets.insert(name, mesh);
        }
    }

    pub fn update_camera(&self, view: CameraUniform) {
        self.gpu_context.queue.write_buffer(
            &self.camera_binding_group.buffer,
            0,
            bytemuck::bytes_of(&view),
        );
    }

    pub fn update_entity_transform(&mut self, entity_id: usize, transform: [[f32; 4]; 4]) {
        let uniform = EntityUniform::new(transform);

        let bind_group = self.entity_bind_groups.entry(entity_id).or_insert_with(|| {
            UniformBindGroup::new(
                &self.gpu_context,
                std::mem::size_of::<EntityUniform>() as u64,
            )
        });

        self.gpu_context
            .queue
            .write_buffer(&bind_group.buffer, 0, bytemuck::bytes_of(&uniform));
    }

    pub fn queue_entity_render(&mut self, model_id: &'static str, entity_id: usize) {
        self.entity_render_queue.push((model_id, entity_id));
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
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

    fn acquire_frame(&mut self) -> Option<SurfaceTexture> {
        match self.gpu_context.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => Some(surface_texture),
            wgpu::CurrentSurfaceTexture::Suboptimal(_) | wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                None
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => None,
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

        self.world_pipeline.draw(
            &mut pass,
            &self.camera_binding_group.bind_group,
            &self.chunk_meshes,
        );

        self.entity_pipeline.draw(
            &mut pass,
            &self.camera_binding_group.bind_group,
            &self.model_assets,
            &self.entity_bind_groups,
            &self.entity_render_queue,
        );
    }
}
