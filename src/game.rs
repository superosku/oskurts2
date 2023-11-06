use crate::camera::Camera;
use crate::constants::{ENTITY_AMOUNT, GROUND_HEIGHT, GROUND_WIDTH};
use crate::entity::Entity;
use crate::entity_container::EntityContainer;
use crate::game_thing::GameThing;
use crate::ground::{Ground, GroundType};
use crate::vec::Vec2f;
use rand::Rng;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
use std::ops::Deref;

pub struct Game {
    entity_container: EntityContainer,
    ground: Ground,
}

impl Game {
    pub fn new() -> Game {
        let mut entities: Vec<Entity> = Vec::new();

        // Spawn 10 entities at random positions in the range of -10, 10
        for _ in 0..ENTITY_AMOUNT {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(1.0..GROUND_WIDTH as f32 - 1.0);
            let y = rng.gen_range(1.0..GROUND_HEIGHT as f32 - 1.0);
            entities.push(Entity::new(Vec2f::new(x, y)));
        }

        let mut entity_container = EntityContainer::new(entities);

        Game {
            entity_container,
            ground: Ground::new(),
        }
    }

    pub fn get_closest_entity_pos(&self, position: &Vec2f, max_radius: f32) -> Option<Vec2f> {
        // match self.entity_container.get_closest_entity(position, max_radius) {
        //     Some(entity) => {
        //         let entity = entity.borrow();
        //         Some(entity.get_position())
        //     },
        //     None => None
        // }

        match (
            self.entity_container
                .get_closest_entity(position, max_radius),
            self.entity_container
                .get_closest_entity_brute_force(position, max_radius),
        ) {
            (Some(e1), Some(e2)) => {
                // Chack that e1 and e2 are the same
                if e1.borrow().get_id() != e2.borrow().get_id() {
                    println!("e1 and e2 are not the same");
                }

                Some(e1.borrow().get_position())
            }
            (None, None) => None,
            _ => {
                println!("This should not happen");
                None
            }
        }
    }

    fn draw_entities(&self, dt: &mut DrawTarget, camera: &Camera, selected_entiy_ids: &Vec<usize>) {
        let mut selection_path_builder = PathBuilder::new();
        let mut goal_path = PathBuilder::new();

        // let mut path_builder = PathBuilder::new();

        let mut path_builder_0 = PathBuilder::new();
        let mut path_builder_1 = PathBuilder::new();
        let mut path_builder_2 = PathBuilder::new();
        let mut path_builder_3 = PathBuilder::new();

        let mut path_builders = [
            &mut path_builder_0,
            &mut path_builder_1,
            &mut path_builder_2,
            &mut path_builder_3,
        ];

        for entity_ref in self.entity_container.iter_all() {
            // entity.draw(dt, camera);
            let entity = entity_ref.borrow();

            let entity_position = entity.get_position();

            let draw_pos = camera.world_to_screen(&Vec2f::new(
                // self.position.x - 0.5,
                // self.position.y - 0.5,
                entity_position.x,
                entity_position.y,
            ));

            let path_builder = &mut path_builders[entity.get_team() as usize];

            path_builder.move_to(draw_pos.x, draw_pos.y);
            path_builder.arc(
                draw_pos.x,
                draw_pos.y,
                camera.length_to_pixels(entity.get_radius()),
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
                    entity_position.x - entity.get_radius(),
                    entity_position.y - entity.get_radius(),
                ));

                selection_path_builder.rect(
                    top_right_corner.x,
                    top_right_corner.y,
                    camera.length_to_pixels(entity.get_radius() * 2.0),
                    camera.length_to_pixels(entity.get_radius() * 2.0),
                );
            }
        }

        // let path_bulider_sources = [
        //     Source::Solid(SolidSource::from_unpremultiplied_argb( 255, 0x7d, 0xde, 0x92, )),
        //     Source::Solid(SolidSource::from_unpremultiplied_argb( 255, 0xde, 0x7d, 0x92, )),
        //     Source::Solid(SolidSource::from_unpremultiplied_argb( 255, 0xde, 0x92, 0x7d, )),
        //     Source::Solid(SolidSource::from_unpremultiplied_argb( 255, 0x92, 0xde, 0x7d, )),
        // ];

        // for i in 0..4 {
        // let path = path_builders[i].finish();
        // let source = &path_bulider_sources[i];
        // let source_dark = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 200, 200, 255));

        dt.fill(
            &path_builder_0.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                255, 0x7d, 0xde, 0x92,
            )),
            &DrawOptions::new(),
        );
        dt.fill(
            &path_builder_1.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                255, 0xde, 0x7d, 0x92,
            )),
            &DrawOptions::new(),
        );
        dt.fill(
            &path_builder_2.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                255, 0xde, 0x92, 0x7d,
            )),
            &DrawOptions::new(),
        );
        dt.fill(
            &path_builder_3.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                255, 0x92, 0xde, 0x7d,
            )),
            &DrawOptions::new(),
        );

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
        let wall_source = Source::Solid(SolidSource::from_unpremultiplied_argb(
            255, 0x89, 0x99, 0xa6,
        ));
        let ground_source = Source::Solid(SolidSource::from_unpremultiplied_argb(
            255, 0x48, 0x40, 0x41,
        ));

        dt.fill(&ground_path, &ground_source, &DrawOptions::new());
        dt.fill(&wall_path, &wall_source, &DrawOptions::new());
    }

    pub fn draw(&self, dt: &mut DrawTarget, camera: &Camera, selected_entiy_ids: &Vec<usize>) {
        self.draw_ground(dt, camera);
        self.draw_entities(dt, camera, selected_entiy_ids);
    }

    pub fn entity_ids_in_bounding_box(&self, top_left: Vec2f, bottom_right: Vec2f) -> Vec<usize> {
        let mut entity_ids: Vec<usize> = Vec::new();
        for entity in self.entity_container.iter_all() {
            let entity_position = entity.borrow().get_position();
            if entity_position.x >= top_left.x
                && entity_position.x <= bottom_right.x
                && entity_position.y >= top_left.y
                && entity_position.y <= bottom_right.y
            {
                entity_ids.push(entity.borrow().get_id());
            }
        }
        entity_ids
    }

    pub fn command_entities_move(&mut self, entity_ids: &Vec<usize>, goal_pos: &Vec2f) {
        // let mut goals = self.ground.generate_goals(goal_pos, entity_ids.len() as i32);
        for entity in self.entity_container.iter_all_mut() {
            if entity_ids.contains(&entity.borrow().get_id()) {
                // let goal = goals.pop().unwrap();
                // entity.set_goal(&goal_pos);
                entity
                    .borrow_mut()
                    .set_goal(&goal_pos, entity_ids.len() as i32);
            }
        }
    }
}

impl GameThing for Game {
    fn update(&mut self) {
        // Make sure entity container is up to date
        self.entity_container.update_entities_by_area();

        // Update entities
        for entity in self.entity_container.iter_all_mut() {
            entity.borrow_mut().update();
        }
        for entity1 in self.entity_container.iter_all() {
            let entity1_position = entity1.borrow().get_position();

            for entity in self
                .entity_container
                .entities_in_radius(&entity1_position, 2.0)
                .iter()
            {
                if entity.borrow().get_id() == entity1.borrow().get_id() {
                    continue;
                }
                let other_position = entity.borrow().get_position();
                let other_radius = entity.borrow().get_radius();
                entity1
                    .borrow_mut()
                    .get_pushed(other_position, other_radius);
            }
        }
        for entity in self.entity_container.iter_all_mut() {
            entity.borrow_mut().collide_with_ground(&self.ground);
            entity.borrow_mut().flip_position();
        }
    }
}
