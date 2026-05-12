use std::sync::Arc;

use wgpu::{SurfaceCapabilities, TextureFormat, TextureView};
use winit::{event_loop::OwnedDisplayHandle, window::Window};

pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub format: wgpu::TextureFormat,
    pub config: wgpu::SurfaceConfiguration,
    pub depth_view: wgpu::TextureView,
}

impl GpuContext {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::POLYGON_MODE_LINE,
                ..Default::default()
            })
            .await?;

        let surface = instance.create_surface(Arc::clone(&window))?;
        let cap = surface.get_capabilities(&adapter);
        let format = Self::get_format(cap);
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            view_formats: vec![format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        let depth_view = Self::create_depth_texture(&device, &config);

        Ok(Self {
            instance,
            device,
            queue,
            surface,
            format,
            config,
            depth_view,
        })
    }

    pub fn new_depth_texture(&mut self) {
        self.depth_view = Self::create_depth_texture(&self.device, &self.config);
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn get_format(cap: SurfaceCapabilities) -> TextureFormat {
        cap.formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(cap.formats[0])
    }
}
