use crate::camera::Camera;
use crate::entity::Entity;
use crate::game_thing::GameThing;
use crate::vec::Vec2f;
use rand::Rng;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

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

        Game { entities: entities }
    }

    pub fn draw(&self, dt: &mut DrawTarget, camera: &Camera) {
        let mut path_builder = PathBuilder::new();

        for entity in self.entities.iter() {
            // entity.draw(dt, camera);
            let entity_position = entity.get_position();

            let draw_pos = camera.world_to_screen(&Vec2f::new(
                // self.position.x - 0.5,
                // self.position.y - 0.5,
                entity_position.x,
                entity_position.y,
            ));

            path_builder.move_to(draw_pos.x, draw_pos.y);
            path_builder.arc(
                draw_pos.x,
                draw_pos.y,
                camera.length_to_pixels(0.5),
                0.0,
                2.0 * std::f32::consts::PI,
            );
        }

        let path = path_builder.finish();
        let source = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 128, 128, 255));
        // let source_dark = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 200, 200, 255));

        dt.fill(&path, &source, &DrawOptions::new());

        // let stroke_style = &mut raqote::StrokeStyle::default();
        // stroke_style.width = camera.length_to_pixels(0.1);
        // dt.stroke(
        //     &path,
        //     &source_dark,
        //     stroke_style,
        //     &DrawOptions::new(),
        // );
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
}
