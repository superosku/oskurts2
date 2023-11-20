use crate::entity::EntityType;
use crate::resources::Resources;
use crate::vec::{Vec2f, Vec2i};

pub enum Event {
    AddRangedProjectile {
        start: Vec2f,
        end: Vec2f,
        team: u8,
    },
    AddMeleeProjectile {
        end: Vec2f,
        team: u8,
    },
    RequestGatherPath {
        entity_id: usize,
        going_towards_resource: bool,
        resource_position: Vec2i,
    },
    IncrementResources {
        team: u8,
        amounts: Resources,
    },
    SpawnEntity {
        entity_type: EntityType,
        position: Vec2f,
        team: u8,
    },
    RequestRePath {
        entity_id: usize,
    },
}

pub struct EventHandler {
    pub events: Vec<Event>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}
