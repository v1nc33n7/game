use crate::renderer::context::*;

pub struct PipelineBuilder<'a> {
    ctx: &'a GpuContext,
    label: Option<&'a str>,
    shader_src: &'a str,
    vertex_entry: &'a str,
    fragment_entry: &'a str,
    vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    bind_group_layouts: &'a [Option<&'a wgpu::BindGroupLayout>],
    polygon_mode: wgpu::PolygonMode,
    depth_stencil: Option<wgpu::DepthStencilState>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(ctx: &'a GpuContext, shader_src: &'a str) -> Self {
        Self {
            ctx,
            label: None,
            shader_src,
            vertex_entry: "vs_main",
            fragment_entry: "fs_main",
            vertex_buffers: &[],
            bind_group_layouts: &[],
            polygon_mode: wgpu::PolygonMode::Fill,
            depth_stencil: None,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn vertex_buffers(mut self, layouts: &'a [wgpu::VertexBufferLayout<'a>]) -> Self {
        self.vertex_buffers = layouts;
        self
    }

    pub fn bind_group_layouts(mut self, layouts: &'a [Option<&'a wgpu::BindGroupLayout>]) -> Self {
        self.bind_group_layouts = layouts;
        self
    }

    #[allow(dead_code)]
    pub fn polygon_mode(mut self, polygon_mode: wgpu::PolygonMode) -> Self {
        self.polygon_mode = polygon_mode;
        self
    }

    pub fn depth_stencil(mut self, depth_stencil: Option<wgpu::DepthStencilState>) -> Self {
        self.depth_stencil = depth_stencil;
        self
    }

    pub fn build(self) -> wgpu::RenderPipeline {
        let shader = self
            .ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: self.label,
                source: wgpu::ShaderSource::Wgsl(self.shader_src.into()),
            });

        let layout = self
            .ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: self.label,
                bind_group_layouts: self.bind_group_layouts,
                immediate_size: 0,
            });

        self.ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: self.label,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(self.vertex_entry),
                    buffers: self.vertex_buffers,
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(self.fragment_entry),
                    compilation_options: Default::default(),
                    targets: &[Some(self.ctx.format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    polygon_mode: self.polygon_mode,
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: self.depth_stencil,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
    }
}
