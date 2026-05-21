use crate::{assets::cube::create_cube_mesh, renderer::Renderer};

pub struct AssetManager;

impl AssetManager {
    pub fn load_static_assets(renderer: &mut Renderer) {
        let (cube_verts, cube_idxs) = create_cube_mesh(1.0, 2.0, 1.0);
        renderer.register_model_asset("cube", &cube_verts, &cube_idxs);

        let (enemy_verts, enemy_idxs) = create_cube_mesh(0.6, 0.6, 0.6);
        renderer.register_model_asset("small_cube", &enemy_verts, &enemy_idxs);
    }
}
