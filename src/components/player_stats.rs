pub struct PlayerStats {
    pub speed: f32,
    pub move_up_force: f32,
}

impl PlayerStats {
    pub fn new(speed: f32, move_up_force: f32) -> Self {
        Self {
            speed,
            move_up_force,
        }
    }
}
