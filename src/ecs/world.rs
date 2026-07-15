use std::collections::HashMap;

use crate::components::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Entity {
    id: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Entity { id }
    }

    pub fn raw(&self) -> u32 {
        self.id
    }
}

#[derive(Default)]
pub struct Entities {
    pub transforms: HashMap<Entity, Transform>,
    pub ai: HashMap<Entity, EnemyAi>,
    pub player_stats: HashMap<Entity, PlayerStats>,
    pub model_ids: HashMap<Entity, &'static str>,
    next_id: u32,
}

impl Entities {
    pub fn spawn(&mut self) -> Entity {
        let id = Entity::new(self.next_id);
        self.next_id += 1;
        id
    }
}
