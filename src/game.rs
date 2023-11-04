use rand::Rng;
use crate::camera::Camera;
use raqote::DrawTarget;
use crate::entity::Entity;
use crate::game_thing::GameThing;
use crate::vec::Vec2f;

pub struct Game {
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Game {
        let mut entities: Vec<Entity> = Vec::new();

        // Spawn 10 entities at random positions in the range of -10, 10
        for _ in 0..50 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-10.0..10.0);
            let y = rng.gen_range(-10.0..10.0);
            entities.push(Entity::new(Vec2f::new(x, y)));
        }

        Game {
            entities: entities
        }
    }
}

impl GameThing for Game {
    fn update(&mut self) {
        // Update entities
        for entity in self.entities.iter_mut() {
            entity.update();
        }
        for i1 in 0..self.entities.len() {
            for i2 in 0..self.entities.len() {
                if i1 == i2 {
                    continue;
                }
                let entity2 = &mut self.entities[i2];
                let entity2_position = entity2.get_position();

                let entity1 = &mut self.entities[i1];
                entity1.get_pushed(entity2_position);
            }
        }
        for entity in self.entities.iter_mut() {
            entity.flip_position();
        }
    }

    fn draw(&self, dt: &mut DrawTarget, camera: &Camera) {
        for entity in self.entities.iter() {
            entity.draw(dt, camera);
        }
    }
}
