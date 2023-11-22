use crate::entity::EntityType;
use crate::event_handler::{Event, EventHandler};
use crate::vec::{Vec2f, Vec2i};

pub struct Building {
    position: Vec2i,
    width: i32,
    height: i32,
    id: usize,
    team: u8,
    spawn_queue: Vec<EntityType>,
    spawn_timer: i32,
    spawn_command_position: Option<Vec2f>,
}

impl Building {
    pub fn new(position: Vec2i, width: i32, height: i32, team: u8) -> Building {
        let random_id = rand::random::<usize>();

        let mut spawn_queue = Vec::new();

        Building {
            position,
            width,
            height,
            id: random_id,
            team,
            spawn_queue,
            spawn_timer: 0,
            spawn_command_position: None,
        }
    }

    pub fn get_spawn_duration(&self) -> i32 {
        if let Some(entity_type) = self.spawn_queue.get(0) {
            match entity_type {
                EntityType::Worker => 50,
                EntityType::Melee => 100,
                EntityType::Ranged => 200,
            }
        } else {
            println!("This thing should not happen");
            0
        }
    }

    pub fn add_to_spawn_queue(&mut self, entity_type: EntityType) {
        self.spawn_queue.push(entity_type);
    }

    pub fn get_spawn_queue(&self) -> &Vec<EntityType> {
        &self.spawn_queue
    }

    pub fn get_spawn_timer(&self) -> i32 {
        self.spawn_timer
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_team(&self) -> u8 {
        self.team
    }

    pub fn get_spawn_position(&self) -> Vec2f {
        Vec2f::new(self.position.x as f32 - 0.5, self.position.y as f32 - 0.5)
    }

    pub fn get_spawn_command_position(&self) -> Option<Vec2f> {
        self.spawn_command_position.clone()
    }

    pub fn set_spawn_command_position(&mut self, position: Vec2f) {
        self.spawn_command_position = Some(position);
    }

    pub fn get_position(&self) -> Vec2i {
        self.position.clone()
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn update(&mut self, event_handler: &mut EventHandler) {
        if !self.spawn_queue.is_empty() {
            self.spawn_timer += 1;

            if self.spawn_timer >= self.get_spawn_duration() {
                self.spawn_timer = 0;

                let entity_type = self.spawn_queue.remove(0);

                event_handler.add_event(Event::SpawnEntity {
                    entity_type,
                    building_id: self.id,
                    team: self.team,
                })
                // TODO: Spawn an entity here
            }
        }
    }
}
