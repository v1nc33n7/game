#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityUniform {
    model: [[f32; 4]; 4],
}

impl EntityUniform {
    pub fn new(model: [[f32; 4]; 4]) -> Self {
        Self { model }
    }
}
