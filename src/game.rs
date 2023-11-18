use crate::building::Building;
use crate::building_container::BuildingContainer;
use crate::camera::Camera;
use crate::constants::{ENTITY_AMOUNT, GROUND_HEIGHT, GROUND_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::entity::{Entity, EntityType};
use crate::entity_container::EntityContainer;
use crate::ground::{Ground, GroundType};
use crate::path_finder::{Path, PathFinder};
use crate::projectile_handler::ProjectileHandler;
use crate::vec::{Vec2f, Vec2i};
use rand::Rng;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Instant;

pub struct Game {
    entity_container: EntityContainer,
    building_container: BuildingContainer,
    pub ground: Ground,
    projectile_handler: ProjectileHandler,
    path_finder: PathFinder,
    pub debug_path: Option<Rc<RefCell<Path>>>,
}

impl Game {
    pub fn new() -> Game {
        let mut entities: Vec<Entity> = Vec::new();

        // Spawn 10 entities at random positions in the range of -10, 10
        // for _ in 0..ENTITY_AMOUNT {
        //     let mut rng = rand::thread_rng();
        //     let x = rng.gen_range(1.0..GROUND_WIDTH as f32 - 1.0);
        //     let y = rng.gen_range(1.0..GROUND_HEIGHT as f32 - 1.0);
        //     entities.push(Entity::new(Vec2f::new(x, y)));
        // }

        for i in 0..200 {
            entities.push(Entity::new_params(
                Vec2f::new(3.0 + i as f32 / 1000.0, 3.0 + i as f32 / 1000.0),
                0,
                0.3,
                EntityType::Meelee,
            ))
        }

        let mut entity_container = EntityContainer::new(entities);

        let mut ground = Ground::new();

        for x in 0..10 {
            for y in 0..10 {
                ground.set_at(8 - 4 + x, 8 - 4 + y, GroundType::Empty);
                ground.set_at(
                    GROUND_WIDTH - 8 - 4 + x,
                    GROUND_HEIGHT - 8 - 4 + y,
                    GroundType::Empty,
                );
            }
        }
        for i in 0..5 {
            ground.set_at(4, 7 + i, GroundType::Gold);
            ground.set_at(GROUND_WIDTH - 9 + i, GROUND_HEIGHT - 4, GroundType::Gold);
        }
        let mut building_container = BuildingContainer::new();
        building_container.add_building(Building::new(Vec2i::new(8, 8), 2, 3, 0), &mut ground);
        building_container.add_building(
            Building::new(Vec2i::new(GROUND_WIDTH - 8, GROUND_HEIGHT - 8), 3, 2, 1),
            &mut ground,
        );

        let start_time = Instant::now();

        let mut path_finder = PathFinder::new();
        // let debug_path =
        //     path_finder.find_path(&ground, Vec2i::new(2, 2), 3, 3, vec![Vec2i::new(10, 10)]);
        // if let Some(path) = &debug_path {
        //     for _ in 0..0 {
        //         path.borrow_mut().do_orienting_round();
        //     }
        // }

        let total_time = start_time.elapsed().as_millis();

        println!("Time to find path: {}ms", total_time);

        Game {
            entity_container,
            building_container,
            ground,
            projectile_handler: ProjectileHandler::new(),
            debug_path: None,
            path_finder,
        }
    }

    fn draw_projectiles(&self, dt: &mut DrawTarget, camera: &Camera) {
        let mut path_builder = PathBuilder::new();

        for projectile in self.projectile_handler.iter() {
            let projectile_position = projectile.get_position();

            let draw_pos =
                camera.world_to_screen(&Vec2f::new(projectile_position.x, projectile_position.y));

            path_builder.move_to(draw_pos.x, draw_pos.y);
            path_builder.arc(
                draw_pos.x,
                draw_pos.y,
                camera.length_to_pixels(0.1),
                0.0,
                2.0 * std::f32::consts::PI,
            );
        }

        dt.fill(
            &path_builder.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                255, 0x00, 0x00, 0x00,
            )),
            &DrawOptions::new(),
        );
    }

    fn draw_buildings(
        &self,
        dt: &mut DrawTarget,
        camera: &Camera,
        selected_building_id: &Option<usize>,
    ) {
        for building in self.building_container.get_buildings().iter() {
            let mut path_builder = PathBuilder::new();
            let mut selection_path_builder = PathBuilder::new();

            let position = building.get_position();
            let width = building.get_width();
            let height = building.get_height();

            let draw_pos = camera.world_to_screen(&Vec2f::new(
                position.x as f32 + 0.1,
                position.y as f32 + 0.1,
            ));
            path_builder.move_to(draw_pos.x, draw_pos.y);
            path_builder.line_to(
                draw_pos.x + camera.length_to_pixels(width as f32 - 0.2),
                draw_pos.y,
            );
            path_builder.line_to(
                draw_pos.x + camera.length_to_pixels(width as f32 - 0.2),
                draw_pos.y + camera.length_to_pixels(height as f32 - 0.2),
            );
            path_builder.line_to(
                draw_pos.x,
                draw_pos.y + camera.length_to_pixels(height as f32 - 0.2),
            );
            path_builder.close();

            let source = if building.get_team() == 0 {
                Source::Solid(SolidSource::from_unpremultiplied_argb(
                    255, 0x7d, 0xde, 0x92,
                ))
            } else {
                Source::Solid(SolidSource::from_unpremultiplied_argb(
                    255, 0xde, 0x7d, 0x92,
                ))
            };

            dt.fill(&path_builder.finish(), &source, &DrawOptions::new());

            if let Some(selected_building_id) = selected_building_id {
                if *selected_building_id == building.get_id() {
                    let selection_draw_pos =
                        camera.world_to_screen(&Vec2f::new(position.x as f32, position.y as f32));
                    selection_path_builder.move_to(selection_draw_pos.x, selection_draw_pos.y);
                    selection_path_builder.line_to(
                        selection_draw_pos.x + camera.length_to_pixels(width as f32),
                        selection_draw_pos.y,
                    );
                    selection_path_builder.line_to(
                        selection_draw_pos.x + camera.length_to_pixels(width as f32),
                        selection_draw_pos.y + camera.length_to_pixels(height as f32),
                    );
                    selection_path_builder.line_to(
                        selection_draw_pos.x,
                        selection_draw_pos.y + camera.length_to_pixels(height as f32),
                    );
                    selection_path_builder.close();

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
                }
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

        let (min_x, max_x, min_y, max_y) = self.get_draw_boundaries(camera);

        for entity_ref in self.entity_container.entities_in_box(
            &Vec2f::new(min_x as f32, min_y as f32),
            &Vec2f::new(max_x as f32, max_y as f32),
            None,
            None,
        ) {
            let entity = entity_ref.borrow();
            let entity_position = entity.get_position();

            let draw_pos =
                camera.world_to_screen(&Vec2f::new(entity_position.x, entity_position.y));

            let path_builder = &mut path_builders[entity.get_team() as usize];
            let radius = entity.get_radius() - 0.0;

            path_builder.move_to(draw_pos.x, draw_pos.y);
            path_builder.arc(
                draw_pos.x,
                draw_pos.y,
                camera.length_to_pixels(radius),
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
                    entity_position.x - radius,
                    entity_position.y - radius,
                ));

                selection_path_builder.rect(
                    top_right_corner.x,
                    top_right_corner.y,
                    camera.length_to_pixels(radius * 2.0),
                    camera.length_to_pixels(radius * 2.0),
                );
            }
        }

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
        );

        for entity_ref in self.entity_container.iter_alive() {
            let entity = entity_ref.borrow();
            let entity_position = entity.get_position();

            let health_ratio = entity.health_ratio();
            if health_ratio < 1.0 {
                let mut path_builder = PathBuilder::new();

                let health_bar_top_left = camera.world_to_screen(&Vec2f::new(
                    entity_position.x - entity.get_radius(),
                    entity_position.y + entity.get_radius(),
                ));
                let health_bar_bottom_right = camera.world_to_screen(&Vec2f::new(
                    entity_position.x + entity.get_radius(),
                    entity_position.y + entity.get_radius() + 0.1,
                ));
                let health_bar_mid_x = health_bar_top_left.x
                    + (health_bar_bottom_right.x - health_bar_top_left.x) * health_ratio;

                let mut path_builder = PathBuilder::new();
                path_builder.move_to(health_bar_top_left.x, health_bar_top_left.y);
                path_builder.line_to(health_bar_mid_x, health_bar_top_left.y);
                path_builder.line_to(health_bar_mid_x, health_bar_bottom_right.y);
                path_builder.line_to(health_bar_top_left.x, health_bar_bottom_right.y);
                path_builder.close();

                let green = (health_ratio * 255.0) as u8;
                let red = ((1.0 - health_ratio) * 255.0) as u8;

                dt.fill(
                    &path_builder.finish(),
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(
                        255, red, green, 0x00,
                    )),
                    &DrawOptions::new(),
                );

                let mut path_builder = PathBuilder::new();
                path_builder.move_to(health_bar_mid_x, health_bar_top_left.y);
                path_builder.line_to(health_bar_bottom_right.x, health_bar_top_left.y);
                path_builder.line_to(health_bar_bottom_right.x, health_bar_bottom_right.y);
                path_builder.line_to(health_bar_mid_x, health_bar_bottom_right.y);
                path_builder.close();

                dt.fill(
                    &path_builder.finish(),
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(
                        255, 0x00, 0x00, 0x00,
                    )),
                    &DrawOptions::new(),
                );
            }
        }
    }

    fn get_draw_boundaries(&self, camera: &Camera) -> (i32, i32, i32, i32) {
        let top_left = camera.screen_to_world(&Vec2f::new(0.0, 0.0));
        let bottom_right =
            camera.screen_to_world(&Vec2f::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));

        let min_x = 0.max(top_left.x as i32);
        let max_x = self.ground.get_width().min(bottom_right.x as i32 + 1);
        let min_y = 0.max(top_left.y as i32);
        let max_y = self.ground.get_height().min(bottom_right.y as i32 + 1);

        (min_x, max_x, min_y, max_y)
    }

    fn draw_ground(&self, dt: &mut DrawTarget, camera: &Camera) {
        let mut ground_path_builder = PathBuilder::new();
        let mut wall_path_builder = PathBuilder::new();
        let mut gold_path_builder = PathBuilder::new();

        let (min_x, max_x, min_y, max_y) = self.get_draw_boundaries(camera);

        for x in min_x..max_x {
            for y in min_y..max_y {
                let ground_type = self.ground.get_at(x, y);
                match ground_type {
                    GroundType::Empty | GroundType::Gold => {
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
                match ground_type {
                    GroundType::Gold => {
                        for (xx, yy) in [
                            // (0.15, 0.0),
                            // (0.85, 0.0),
                            // (0.15, 0.7),
                            // (0.85, 0.7),
                            (0.15 + 0.2, 0.0 + 0.1),
                            (0.85 - 0.1, 0.0 + 0.2),
                            (0.15 + 0.2, 0.7 - 0.3),
                            (0.85 - 0.2, 0.7 - 0.1),
                        ] {
                            let draw_pos =
                                camera.world_to_screen(&Vec2f::new(x as f32 + xx, y as f32 + yy));
                            gold_path_builder.move_to(draw_pos.x, draw_pos.y);
                            gold_path_builder.line_to(
                                draw_pos.x + camera.length_to_pixels(0.15),
                                draw_pos.y + camera.length_to_pixels(0.3),
                            );
                            gold_path_builder.line_to(
                                draw_pos.x - camera.length_to_pixels(0.15),
                                draw_pos.y + camera.length_to_pixels(0.3),
                            );
                            // gold_path_builder.line_to(draw_pos.x, draw_pos.y + camera.length_to_pixels(1.0));
                            gold_path_builder.close();
                        }
                    }
                    _ => {}
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
        let gold_source = Source::Solid(SolidSource::from_unpremultiplied_argb(
            255, 0xff, 0xd7, 0x00,
        ));

        dt.fill(&ground_path, &ground_source, &DrawOptions::new());
        dt.fill(&wall_path, &wall_source, &DrawOptions::new());
        dt.fill(
            &gold_path_builder.finish(),
            &gold_source,
            &DrawOptions::new(),
        );
    }

    pub fn draw_debug_path(&self, dt: &mut DrawTarget, camera: &Camera) {
        if let Some(debug_path) = &self.debug_path {
            let mut path_builder = PathBuilder::new();

            let (min_x, max_x, min_y, max_y) = self.get_draw_boundaries(camera);

            let debug_path = debug_path.borrow();
            for x in min_x..max_x {
                for y in min_y..max_y {
                    let path_item = (x, y);
                    if let Some(direction) = debug_path.position_datas.get(&path_item) {
                        let center_pos = camera.world_to_screen(&Vec2f::new(
                            path_item.0 as f32 + 0.5,
                            path_item.1 as f32 + 0.5,
                        ));

                        path_builder.move_to(
                            center_pos.x - camera.length_to_pixels(direction.x * 0.2),
                            center_pos.y - camera.length_to_pixels(direction.y * 0.2),
                        );
                        path_builder.line_to(
                            center_pos.x + camera.length_to_pixels(direction.x * 0.2),
                            center_pos.y + camera.length_to_pixels(direction.y * 0.2),
                        );
                        path_builder.line_to(
                            center_pos.x
                                + camera.length_to_pixels(
                                    direction.x * 0.2 + direction.y * 0.1 - direction.x * 0.1,
                                ),
                            center_pos.y
                                + camera.length_to_pixels(
                                    direction.y * 0.2 - direction.x * 0.1 - direction.y * 0.1,
                                ),
                        );
                        path_builder.move_to(
                            center_pos.x + camera.length_to_pixels(direction.x * 0.2),
                            center_pos.y + camera.length_to_pixels(direction.y * 0.2),
                        );
                        path_builder.line_to(
                            center_pos.x
                                + camera.length_to_pixels(
                                    direction.x * 0.2 - direction.y * 0.1 - direction.x * 0.1,
                                ),
                            center_pos.y
                                + camera.length_to_pixels(
                                    direction.y * 0.2 + direction.x * 0.1 - direction.y * 0.1,
                                ),
                        );
                    }
                }
            }

            let path = path_builder.finish();
            let source = Source::Solid(SolidSource::from_unpremultiplied_argb(
                255, 0x38, 0x30, 0x31,
            ));
            let stroke_style = &mut raqote::StrokeStyle::default();
            dt.stroke(&path, &source, &stroke_style, &DrawOptions::new());
        }
    }

    pub fn draw(
        &self,
        dt: &mut DrawTarget,
        camera: &Camera,
        selected_entiy_ids: &Vec<usize>,
        selected_building_id: &Option<usize>,
    ) {
        self.draw_ground(dt, camera);
        self.draw_entities(dt, camera, selected_entiy_ids);
        self.draw_buildings(dt, camera, selected_building_id);
        self.draw_projectiles(dt, camera);
        self.draw_debug_path(dt, camera);
    }

    pub fn entity_ids_in_bounding_box(&self, top_left: &Vec2f, bottom_right: &Vec2f) -> Vec<usize> {
        let mut entity_ids: Vec<usize> = Vec::new();
        for entity in self.entity_container.iter_alive() {
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

    pub fn first_building_id_in_bouding_box(
        &self,
        top_left: &Vec2f,
        bottom_right: &Vec2f,
    ) -> Option<usize> {
        for building in self.building_container.get_buildings().iter() {
            let pos = building.get_position().as_vec2f();
            let width = building.get_width() as f32;
            let height = building.get_height() as f32;

            if bottom_right.x > pos.x
                && bottom_right.y > pos.y
                && top_left.x < pos.x + width
                && top_left.y < pos.y + height
            {
                return Some(building.get_id());
            }
        }
        None
    }

    pub fn command_building_spawn(&mut self, building_id: usize) {
        for building in self.building_container.get_buildings().iter() {
            if building.get_id() == building_id {
                self.entity_container.spawn_entity(Entity::new_params(
                    building.get_spawn_position(),
                    building.get_team(),
                    0.25,
                    EntityType::Ranged,
                ));
            }
        }
    }

    pub fn command_entities_move(
        &mut self,
        entity_ids: &Vec<usize>,
        goal_pos: &Vec2f,
        is_attack_command: bool,
    ) {
        // TODO: Define if is gather from map position clicked

        // let mut goals = self.ground.generate_goals(goal_pos, entity_ids.len() as i32);

        let is_gather_command = match self.ground.get_pos(goal_pos) {
            GroundType::Gold => true,
            _ => false,
        };

        let entity_positions_iter = self
            .entity_container
            .iter_alive()
            .filter(|entity| entity_ids.contains(&entity.borrow().get_id()))
            // entity_ids
            // .iter()
            .map(|entity| entity.borrow().get_position().as_vec2i());
        let entity_positions: HashSet<Vec2i> = HashSet::from_iter(entity_positions_iter);

        let found_path =
            self.path_finder
                .find_path(&self.ground, goal_pos.as_vec2i(), 1, 1, &entity_positions);

        self.debug_path = found_path.clone();

        if let Some(found_path) = found_path {
            let entities_commanded: Vec<&Rc<RefCell<Entity>>> = self
                .entity_container
                .iter_alive()
                .filter(|entity| entity_ids.contains(&entity.borrow().get_id()))
                .collect();

            let entity_mass = entities_commanded
                .iter()
                .map(|entity| {
                    let radius = entity.borrow().get_radius();
                    radius * radius * 4.0
                })
                .sum::<f32>();

            for entity in entities_commanded {
                if is_gather_command {
                    entity
                        .borrow_mut()
                        .set_action_gather(&goal_pos, &Vec2f::new(5.0, 5.0), 0);
                } else if is_attack_command {
                    entity.borrow_mut().set_action_attack(
                        found_path.clone(),
                        &goal_pos,
                        entity_mass,
                    );
                } else {
                    entity
                        .borrow_mut()
                        .set_action_move(found_path.clone(), &goal_pos, entity_mass);
                }
            }
        }
    }

    pub fn update(&mut self) {
        // Make sure entity container is up to date
        self.entity_container.update_entities_by_area();

        // // Update entities
        // for entity in self.entity_container.iter_alive() {
        //     let closest_enemy = self.entity_container.get_closest_entity(
        //         &entity.borrow().get_position(),
        //         8.0,
        //         None,
        //         Some(entity.borrow().get_team()),
        //     );
        //
        //     entity.borrow_mut().update(
        //         closest_enemy,
        //         &mut self.projectile_handler,
        //     );
        // }

        let mut entity_close: Vec<(
            &Rc<RefCell<Entity>>,
            Option<Rc<RefCell<Entity>>>,
            Vec<Rc<RefCell<Entity>>>,
        )> = Vec::new();

        for entity1 in self.entity_container.iter_alive() {
            let entity1_position = entity1.borrow().get_position();

            let closest_enemy = self.entity_container.get_closest_entity(
                &entity1.borrow().get_position(),
                8.0,
                None,
                Some(entity1.borrow().get_team()),
            );

            let mut close_entities: Vec<Rc<RefCell<Entity>>> = Vec::new();
            for entity in self
                .entity_container
                .entities_in_radius(&entity1_position, 2.0, None, None)
                .iter()
            {
                if entity.borrow().get_id() == entity1.borrow().get_id() {
                    continue;
                }
                close_entities.push(entity.clone());
            }
            entity_close.push((entity1, closest_enemy, close_entities));
        }

        let steps = 4;
        let step_delta = 1.0 / steps as f32;

        for _ in 0..steps {
            for (entity1, closest_enemy, close_entities) in entity_close.iter() {
                // Update entities
                entity1.borrow_mut().update(
                    closest_enemy.clone(),
                    &mut self.projectile_handler,
                    step_delta,
                );

                // Entities push each other
                for entity in close_entities {
                    // for entity in self
                    //     .entity_container
                    //     .entities_in_radius(&entity1_position, 2.0, None, None)
                    //     .iter()
                    // {
                    if entity.borrow().get_id() == entity1.borrow().get_id() {
                        continue;
                    }
                    let other_position = entity.borrow().get_position();
                    let other_radius = entity.borrow().get_radius();
                    entity1
                        .borrow_mut()
                        .get_pushed(other_position, other_radius, 1.0);
                }

                // Entities collide with ground
                entity1.borrow_mut().collide_with_ground(&self.ground, 1.0);

                // Flip updated position of each entity (should be done last after each move)
                entity1.borrow_mut().flip_position();
            }
        }

        // Update projectiles
        self.projectile_handler.progress_projectiles();
        for projectile in self.projectile_handler.get_impacting_projectiles() {
            if let Some(entity_hit) = self.entity_container.get_closest_entity(
                &projectile.get_position(),
                1.0,
                None,
                None, // TODO: Projectile should have team and we should filter out team members
            ) {
                entity_hit.borrow_mut().take_damage(projectile.get_damage());
            }
        }
        self.projectile_handler.remove_impacting_projectiles(); // Since impacting projectiles have been handled, remove them

        // Remove dead entities
        self.entity_container.remove_dead();
    }
}
