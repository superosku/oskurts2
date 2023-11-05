use crate::camera::Camera;
use crate::entity::Entity;
use crate::game_thing::GameThing;
use crate::ground::{Ground, GroundType};
use crate::vec::Vec2f;
use rand::Rng;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

pub struct Game {
    entities: Vec<Entity>,
    ground: Ground,
}

impl Game {
    pub fn new() -> Game {
        let mut entities: Vec<Entity> = Vec::new();

        // Spawn 10 entities at random positions in the range of -10, 10
        for _ in 0..50 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0.0..20.0);
            let y = rng.gen_range(0.0..20.0);
            entities.push(Entity::new(Vec2f::new(x, y)));
        }

        Game {
            entities,
            ground: Ground::new(),
        }
    }

    fn draw_entities(&self, dt: &mut DrawTarget, camera: &Camera, selected_entiy_ids: &Vec<usize>) {
        let mut path_builder = PathBuilder::new();
        let mut selection_path_builder = PathBuilder::new();
        let mut goal_path = PathBuilder::new();

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

            if selected_entiy_ids.contains(&entity.get_id()) {
                if let Some(goal) = entity.get_goal() {
                    let goal_pos = camera.world_to_screen(&Vec2f::new(goal.x, goal.y));

                    goal_path.move_to(draw_pos.x, draw_pos.y);
                    goal_path.line_to(goal_pos.x, goal_pos.y);
                }

                let top_right_corner = camera.world_to_screen(&Vec2f::new(
                    entity_position.x - 0.5,
                    entity_position.y - 0.5,
                ));

                selection_path_builder.rect(
                    top_right_corner.x,
                    top_right_corner.y,
                    camera.length_to_pixels(1.0),
                    camera.length_to_pixels(1.0),
                );
            }
        }

        let path = path_builder.finish();
        let source = Source::Solid(SolidSource::from_unpremultiplied_argb(128, 128, 128, 255));
        // let source_dark = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 200, 200, 255));

        dt.fill(&path, &source, &DrawOptions::new());

        let selection_path = selection_path_builder.finish();
        let selection_source =
            Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 255, 0));

        let stroke_style = &mut raqote::StrokeStyle::default();
        dt.stroke(
            &selection_path,
            &selection_source,
            stroke_style,
            &DrawOptions::new(),
        );

        dt.stroke(
            &goal_path.finish(),
            &selection_source,
            stroke_style,
            &DrawOptions::new(),
        )

        // let stroke_style = &mut raqote::StrokeStyle::default();
        // stroke_style.width = camera.length_to_pixels(0.1);
        // dt.stroke(
        //     &path,
        //     &source_dark,
        //     stroke_style,
        //     &DrawOptions::new(),
        // );
    }

    fn draw_ground(&self, dt: &mut DrawTarget, camera: &Camera) {
        let mut ground_path_builder = PathBuilder::new();
        let mut wall_path_builder = PathBuilder::new();

        for x in 0..self.ground.get_width() {
            for y in 0..self.ground.get_height() {
                match self.ground.get_at(x, y) {
                    GroundType::Empty => {
                        let draw_pos = camera.world_to_screen(&Vec2f::new(x as f32, y as f32));
                        ground_path_builder.move_to(draw_pos.x, draw_pos.y);
                        ground_path_builder
                            .line_to(draw_pos.x + camera.length_to_pixels(1.0), draw_pos.y);
                        ground_path_builder.line_to(
                            draw_pos.x + camera.length_to_pixels(1.0),
                            draw_pos.y + camera.length_to_pixels(1.0),
                        );
                        ground_path_builder
                            .line_to(draw_pos.x, draw_pos.y + camera.length_to_pixels(1.0));
                        ground_path_builder.close();
                    }
                    GroundType::Wall => {
                        let draw_pos = camera.world_to_screen(&Vec2f::new(x as f32, y as f32));
                        wall_path_builder.move_to(draw_pos.x, draw_pos.y);
                        wall_path_builder
                            .line_to(draw_pos.x + camera.length_to_pixels(1.0), draw_pos.y);
                        wall_path_builder.line_to(
                            draw_pos.x + camera.length_to_pixels(1.0),
                            draw_pos.y + camera.length_to_pixels(1.0),
                        );
                        wall_path_builder
                            .line_to(draw_pos.x, draw_pos.y + camera.length_to_pixels(1.0));
                        wall_path_builder.close();
                    }
                }
            }
        }

        let ground_path = ground_path_builder.finish();
        let wall_path = wall_path_builder.finish();
        let ground_source = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 128, 64, 32));
        let wall_source = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 100, 100, 100));

        dt.fill(&ground_path, &ground_source, &DrawOptions::new());
        dt.fill(&wall_path, &wall_source, &DrawOptions::new());
    }

    pub fn draw(&self, dt: &mut DrawTarget, camera: &Camera, selected_entiy_ids: &Vec<usize>) {
        self.draw_ground(dt, camera);
        self.draw_entities(dt, camera, selected_entiy_ids);
    }

    pub fn entity_ids_in_bounding_box(&self, top_left: Vec2f, bottom_right: Vec2f) -> Vec<usize> {
        let mut entity_ids: Vec<usize> = Vec::new();
        for entity in self.entities.iter() {
            let entity_position = entity.get_position();
            if entity_position.x >= top_left.x
                && entity_position.x <= bottom_right.x
                && entity_position.y >= top_left.y
                && entity_position.y <= bottom_right.y
            {
                entity_ids.push(entity.get_id());
            }
        }
        entity_ids
    }

    pub fn command_entities_move(&mut self, entity_ids: &Vec<usize>, goal_pos: &Vec2f) {
        // let mut goals = self.ground.generate_goals(goal_pos, entity_ids.len() as i32);
        for entity in self.entities.iter_mut() {
            if entity_ids.contains(&entity.get_id()) {
                // let goal = goals.pop().unwrap();
                // entity.set_goal(&goal_pos);
                entity.set_goal(&goal_pos, entity_ids.len() as i32);
            }
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
            entity.collide_with_ground(&self.ground);
            entity.flip_position();
        }
    }
}
