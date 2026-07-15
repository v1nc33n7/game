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
        let id = self.next_id;
        self.next_id = id.checked_add(1).expect("entity id space exhausted");
        Entity::new(id)
    }

    pub fn spawn_builder(&mut self) -> EntityBuilder<'_> {
        let id = self.spawn();
        EntityBuilder { entities: self, id }
    }
}

pub struct EntityBuilder<'a> {
    entities: &'a mut Entities,
    id: Entity,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_transform(self, transform: Transform) -> Self {
        self.entities.transforms.insert(self.id, transform);
        self
    }

    pub fn with_model_id(self, model_id: &'static str) -> Self {
        self.entities.model_ids.insert(self.id, model_id);
        self
    }

    pub fn with_ai(self, ai: EnemyAi) -> Self {
        self.entities.ai.insert(self.id, ai);
        self
    }

    pub fn with_player_stats(self, stats: PlayerStats) -> Self {
        self.entities.player_stats.insert(self.id, stats);
        self
    }

    pub fn id(self) -> Entity {
        self.id
    }
}
