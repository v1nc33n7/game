use crate::renderer::Vertex;

pub fn create_cube_mesh(width: f32, height: f32, depth: f32) -> (Vec<Vertex>, Vec<u32>) {
    let w = width / 2.0;
    let d = depth / 2.0;

    let bottom_y = 0.0;
    let top_y = height;

    let positions = [
        [-w, bottom_y, d],
        [w, bottom_y, d],
        [w, top_y, d],
        [-w, top_y, d],
        [-w, bottom_y, -d],
        [w, bottom_y, -d],
        [w, top_y, -d],
        [-w, top_y, -d],
    ];

    let debug_color = [0.92, 0.40, 0.33];

    let vertices: Vec<Vertex> = positions
        .iter()
        .map(|pos| Vertex::new(*pos, debug_color))
        .collect();

    #[rustfmt::skip]
    let indices = vec![
        0, 1, 2,  0, 2, 3,
        1, 5, 6,  1, 6, 2,
        5, 4, 7,  5, 7, 6,
        4, 0, 3,  4, 3, 7,
        3, 2, 6,  3, 6, 7,
        4, 5, 1,  4, 1, 0,
    ];

    (vertices, indices)
}
