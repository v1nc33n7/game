#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(view: [[f32; 4]; 4]) -> Self {
        Self { view }
    }
}
