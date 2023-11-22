use crate::building::Building;
use crate::building_container::BuildingContainer;
use crate::camera::Camera;
use crate::constants::{ENTITY_AMOUNT, GROUND_HEIGHT, GROUND_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::draw::draw_health_bar;
use crate::entity::{BuildGoal, Entity, EntityAction, EntityType};
use crate::entity_container::EntityContainer;
use crate::event_handler::{Event, EventHandler};
use crate::ground::{Ground, GroundType};
use crate::path_finder::{Path, PathFinder, PathGoal};
use crate::projectile_handler::ProjectileHandler;
use crate::resources::Resources;
use crate::team::Team;
use crate::vec::{Vec2f, Vec2i};
use rand::Rng;
use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source};
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
    teams: Vec<Team>,
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

        for i in 0..20 {
            entities.push(Entity::new_params(
                Vec2f::new(3.0 + i as f32 / 1000.0, 3.0 + i as f32 / 1000.0),
                0,
                EntityType::Melee,
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
        building_container
            .add_building(Building::new(Vec2i::new(8, 8), 2, 3, 0, true), &mut ground);
        building_container.add_building(
            Building::new(
                Vec2i::new(GROUND_WIDTH - 8, GROUND_HEIGHT - 8),
                3,
                2,
                1,
                true,
            ),
            &mut ground,
        );

        let start_time = Instant::now();

        let mut path_finder = PathFinder::new();

        let total_time = start_time.elapsed().as_millis();

        println!("Time to find path: {}ms", total_time);

        let teams = vec![Team::new(0), Team::new(1)];

        Game {
            entity_container,
            building_container,
            ground,
            projectile_handler: ProjectileHandler::new(),
            debug_path: None,
            path_finder,
            teams,
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
        for building_ref in self.building_container.get_buildings().iter() {
            let building = building_ref.borrow();

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

            if building.is_constructed() {
                dt.fill(&path_builder.finish(), &source, &DrawOptions::new());
            } else {
                dt.stroke(
                    &path_builder.finish(),
                    &source,
                    &mut raqote::StrokeStyle::default(),
                    // stroke_style,
                    &DrawOptions::new(),
                    //     &selection_path,
                    // &selection_source,
                    // stroke_style,
                    // &DrawOptions::new(),
                );
            }

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

            let health_ratio = if building.is_constructed() {
                building.health.health_ratio()
            } else {
                building.construction_progress.health_ratio()
            };
            draw_health_bar(
                dt,
                camera,
                health_ratio,
                &(building.get_position().as_vec2f()
                    + Vec2f::new(
                        building.get_width() as f32 / 2.0,
                        building.get_height() as f32 + 0.1,
                    )),
                building.get_width() as f32,
            );
        }
    }

    fn draw_entities(&self, dt: &mut DrawTarget, camera: &Camera, selected_entiy_ids: &Vec<usize>) {
        let mut selection_path_builder = PathBuilder::new();
        let mut goal_path = PathBuilder::new();
        let mut entity_type_path_builder = PathBuilder::new();

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

            let delt = camera.length_to_pixels(radius) * 0.5;

            match entity.get_entity_type() {
                EntityType::Ranged => {
                    entity_type_path_builder.move_to(draw_pos.x, draw_pos.y - delt);
                    entity_type_path_builder
                        .line_to(draw_pos.x + delt * 0.81, draw_pos.y + delt * 0.58);
                    entity_type_path_builder
                        .line_to(draw_pos.x - delt * 0.81, draw_pos.y + delt * 0.58);
                    entity_type_path_builder.close();
                }
                EntityType::Melee => {
                    entity_type_path_builder
                        .move_to(draw_pos.x - delt * 0.707, draw_pos.y - delt * 0.707);
                    entity_type_path_builder
                        .line_to(draw_pos.x + delt * 0.707, draw_pos.y - delt * 0.707);
                    entity_type_path_builder
                        .line_to(draw_pos.x + delt * 0.707, draw_pos.y + delt * 0.707);
                    entity_type_path_builder
                        .line_to(draw_pos.x - delt * 0.707, draw_pos.y + delt * 0.707);
                    entity_type_path_builder.close();
                }
                EntityType::Worker => {
                    entity_type_path_builder
                        .move_to(draw_pos.x - delt * 0.707, draw_pos.y - delt * 0.2);
                    entity_type_path_builder
                        .line_to(draw_pos.x + delt * 0.707, draw_pos.y - delt * 0.2);
                    entity_type_path_builder
                        .line_to(draw_pos.x + delt * 0.707, draw_pos.y + delt * 0.2);
                    entity_type_path_builder
                        .line_to(draw_pos.x - delt * 0.707, draw_pos.y + delt * 0.2);
                    entity_type_path_builder.close();
                }
            }

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

        let entity_type_path = entity_type_path_builder.finish();
        dt.fill(
            &entity_type_path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 255, 255)),
            // stroke_style,
            &DrawOptions::new(),
        );

        for entity_ref in self.entity_container.iter_alive() {
            let entity = entity_ref.borrow();
            let entity_position = entity.get_position();

            let health_ratio = entity.health.health_ratio();
            if health_ratio < 1.0 {
                draw_health_bar(
                    dt,
                    camera,
                    health_ratio,
                    &(entity_position + Vec2f::new(0.0, entity.get_radius() + 0.1)),
                    entity.get_radius() * 2.0,
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
        self.draw_overlay(dt, camera, selected_entiy_ids, selected_building_id);
    }

    pub fn draw_overlay(
        &self,
        dt: &mut DrawTarget,
        camera: &Camera,
        selected_entiy_ids: &Vec<usize>,
        selected_building_id: &Option<usize>,
    ) {
        let font = font_kit::loader::Loader::from_file(
            &mut std::fs::File::open("res/Roboto-Medium.ttf").unwrap(),
            0,
        )
        .unwrap();

        for (i, team) in self.teams.iter().enumerate() {
            dt.draw_text(
                &font,
                20.,
                &format!("Team {} Gold: {}", team.get_id(), team.get_resources().gold),
                // "3",
                Point::new(0., 20. + 20. * i as f32),
                &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 255, 255)),
                &DrawOptions::new(),
            );
        }

        dt.fill_rect(
            0.,
            SCREEN_HEIGHT as f32 - 180.,
            SCREEN_WIDTH as f32,
            180.,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 128, 128, 128)),
            &DrawOptions::new(),
        );

        dt.fill_rect(
            10.,
            SCREEN_HEIGHT as f32 - 170.,
            160.,
            160.,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 64, 64, 64)),
            &DrawOptions::new(),
        );

        dt.draw_text(
            &font,
            20.,
            &format!("Minimap here"),
            Point::new(10., SCREEN_HEIGHT as f32 - 170. + 20.),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 255, 255)),
            &DrawOptions::new(),
        );

        if let Some(selected_building_id) = selected_building_id {
            if let Some(building_ref) = self
                .building_container
                .get_building_by_id(*selected_building_id)
            {
                let building = building_ref.borrow();

                if building.is_constructed() {
                    let spawn_queue = building.get_spawn_queue().clone();
                    let spawn_timer = building.get_spawn_timer();
                    for (i, spawn_item) in spawn_queue.iter().enumerate() {
                        dt.draw_text(
                            &font,
                            20.,
                            &format!("{:?}", spawn_item),
                            Point::new(180., SCREEN_HEIGHT as f32 - 170. + 20. + 20. * i as f32),
                            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                                255, 255, 255, 255,
                            )),
                            &DrawOptions::new(),
                        );
                    }
                    if !spawn_queue.is_empty() {
                        let spawn_duratoin = building.get_spawn_duration();
                        dt.fill_rect(
                            180. + 100.,
                            SCREEN_HEIGHT as f32 - 170.,
                            100.,
                            20.,
                            &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 64, 64, 64)),
                            &DrawOptions::new(),
                        );
                        dt.fill_rect(
                            180. + 100.,
                            SCREEN_HEIGHT as f32 - 170.,
                            100. * (spawn_timer as f32 / spawn_duratoin as f32),
                            20.,
                            &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 255, 0)),
                            &DrawOptions::new(),
                        );
                    }
                } else {
                    dt.draw_text(
                        &font,
                        20.,
                        &format!(
                            "Under construction: {}%",
                            (building.construction_progress.health_ratio() * 100.0) as i32
                        ),
                        Point::new(180., SCREEN_HEIGHT as f32 - 170. + 20. + 20.),
                        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 255, 255)),
                        &DrawOptions::new(),
                    );
                }
            }
        }
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
            let pos = building.borrow().get_position().as_vec2f();
            let width = building.borrow().get_width() as f32;
            let height = building.borrow().get_height() as f32;

            if bottom_right.x > pos.x
                && bottom_right.y > pos.y
                && top_left.x < pos.x + width
                && top_left.y < pos.y + height
            {
                return Some(building.borrow().get_id());
            }
        }
        None
    }

    fn decrement_team_resources(&mut self, team_id: u8, amount: Resources) -> bool {
        for team in self.teams.iter_mut() {
            if team.get_id() == team_id {
                if team.get_resources().can_afford(&amount) {
                    team.decrement_resources(&amount);
                    return true;
                }
                return false;
            }
        }
        return false;
    }

    pub fn command_building_spawn(&mut self, building_id: usize, entity_type: EntityType) {
        if let Some(building) = self.building_container.get_building_by_id(building_id) {
            let team = building.borrow().get_team();

            if self.decrement_team_resources(team, Resources::new(80)) {
                self.building_container
                    .add_to_building_spawn_queue(building_id, entity_type);
            }
        } else {
            println!("Building with id {} not found", building_id);
        }
        // self.building_container.get_building_by_id(building_id).unwrap().add_to_spawn_queue(entity_type);
        // for building in self.building_container.get_buildings().iter() {
        //     if building.get_id() == building_id {
        //         self.entity_container.spawn_entity(Entity::new_params(
        //             building.get_spawn_position(),
        //             building.get_team(),
        //             EntityType::Worker,
        //         ));
        //     }
        // }
    }

    pub fn command_entities_simple(
        &mut self,
        entity_ids: &Vec<usize>,
        is_idle: bool,
        is_hold: bool,
    ) {
        let entities_commanded: Vec<&Rc<RefCell<Entity>>> = self
            .entity_container
            .iter_alive()
            .filter(|entity| entity_ids.contains(&entity.borrow().get_id()))
            .collect();

        for entity in entities_commanded {
            if is_idle {
                entity.borrow_mut().set_action_idle();
            }
            if is_hold {
                entity.borrow_mut().set_action_hold();
            }
        }
    }

    pub fn set_spawn_command_position(&mut self, building_id: usize, pos: &Vec2f) {
        self.building_container
            .set_spawn_command_position(building_id, pos);
    }

    pub fn command_construct_building(
        &mut self,
        entity_ids: &Vec<usize>,
        building_top_left: &Vec2i,
    ) {
        let mut team_id: Option<u8> = None;

        let mut entities = Vec::new();

        for entity_ref in self.entity_container.iter_alive() {
            let entity = entity_ref.borrow();
            if entity_ids.contains(&entity.get_id()) && entity.is_worker() {
                let entity_team_id = entity.get_team();
                if let Some(team_id) = team_id {
                    if team_id != entity_team_id {
                        println!(
                            "Cannot command entities from different teams to construct a building"
                        );
                        return;
                    }
                }
                team_id = Some(entity_team_id);

                entities.push(entity_ref.clone());
            }
        }

        if let Some(team_id) = team_id {
            for x in 0..2 {
                for y in 0..2 {
                    if self
                        .ground
                        .blocked_at(building_top_left.x + x, building_top_left.y + y)
                    {
                        println!("Cannot construct building on non-empty tile");
                        return;
                    }
                }
            }

            if self.decrement_team_resources(team_id, Resources::new(100)) {
                let new_building_width = 2;
                let new_building_height = 2;

                let new_building_ref = self.building_container.add_building(
                    Building::new(
                        building_top_left.clone(),
                        new_building_width,
                        new_building_height,
                        team_id,
                        false,
                    ),
                    &mut self.ground,
                );

                let new_path = self.path_finder.find_path(
                    &self.ground,
                    PathGoal::Rect {
                        pos: building_top_left.clone(),
                        size: Vec2i::new(new_building_width, new_building_height),
                    },
                    &HashSet::from_iter(
                        entities
                            .iter()
                            .map(|entity| entity.borrow().get_position().as_vec2i()),
                    ),
                );

                for entity in entities.iter() {
                    entity
                        .borrow_mut()
                        .set_action_build(new_building_ref.clone(), new_path.clone());
                    // entity.borrow_mut().set_action_construct_building(new_building_ref.clone());
                }
            } else {
                println!("Not enough resources to construct building");
            }
        } else {
            println!("No entities selected to construct building (that is worker)");
        }
    }

    pub fn command_entities_move(
        &mut self,
        entity_ids: &Vec<usize>,
        goal_pos: &Vec2f,
        is_attack_command: bool,
    ) {
        enum MoveGoalType {
            Move,
            Attack,
            Gather,
            Build(Rc<RefCell<Building>>),
        }

        // TODO: Make sure only one team entities are selected
        // TODO: Get the team id and filter buildings by it

        let move_goal_type: MoveGoalType = if is_attack_command {
            MoveGoalType::Attack
        } else if self.ground.get_pos(goal_pos) == GroundType::Gold {
            MoveGoalType::Gather
        } else if let Some(building) = self.building_container.get_building_at(
            &goal_pos.as_vec2i(),
            None, // TODO: Filter here by team id
            None,
        ) {
            MoveGoalType::Build(building)
        } else {
            MoveGoalType::Move
        };

        // let is_gather_command = match self.ground.get_pos(goal_pos) {
        //     GroundType::Gold => true,
        //     _ => false,
        // };

        let entity_positions_iter = self
            .entity_container
            .iter_alive()
            .filter(|entity| entity_ids.contains(&entity.borrow().get_id()))
            .map(|entity| entity.borrow().get_position().as_vec2i());
        let entity_positions: HashSet<Vec2i> = HashSet::from_iter(entity_positions_iter);

        let path_goal = match &move_goal_type {
            MoveGoalType::Move | MoveGoalType::Attack => PathGoal::Point {
                pos: goal_pos.clone(),
            },
            MoveGoalType::Gather => PathGoal::Rect {
                pos: goal_pos.as_vec2i(),
                size: Vec2i::new(1, 1),
            },
            MoveGoalType::Build(building) => PathGoal::Rect {
                pos: building.borrow().get_position().clone(),
                size: Vec2i::new(
                    building.borrow().get_width(),
                    building.borrow().get_height(),
                ),
            },
        };

        let found_path = self
            .path_finder
            .find_path(&self.ground, path_goal, &entity_positions);

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
                let entity_type = entity.borrow().get_entity_type();

                match &move_goal_type {
                    MoveGoalType::Move => {
                        entity.borrow_mut().set_action_move(
                            found_path.clone(),
                            &goal_pos,
                            entity_mass,
                        );
                    }
                    MoveGoalType::Attack => {
                        entity.borrow_mut().set_action_attack(
                            found_path.clone(),
                            &goal_pos,
                            entity_mass,
                        );
                    }
                    MoveGoalType::Gather => match entity_type {
                        EntityType::Worker => {
                            entity.borrow_mut().set_action_gather(
                                goal_pos.as_vec2i(),
                                0,
                                Some(found_path.clone()),
                            );
                        }
                        _ => {}
                    },
                    MoveGoalType::Build(building) => {
                        entity
                            .borrow_mut()
                            .set_action_build(building.clone(), Some(found_path.clone()));
                    }
                };
            }
        }
    }

    pub fn update(&mut self) {
        // Make sure entity container is up to date
        self.entity_container.update_entities_by_area();

        let mut entity_close: Vec<(
            &Rc<RefCell<Entity>>,
            Option<Rc<RefCell<Entity>>>,
            Vec<Rc<RefCell<Entity>>>,
            Option<Rc<RefCell<Building>>>,
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

            let closest_enemy_building = self.building_container.get_closest_building(
                &entity1.borrow().get_position().as_vec2i(),
                None,
                Some(entity1.borrow().get_team()),
                8.0,
            );

            entity_close.push((
                entity1,
                closest_enemy,
                close_entities,
                closest_enemy_building,
            ));
        }

        let steps = 4;
        let step_delta = 1.0 / steps as f32;

        let mut event_handler = EventHandler::new();

        for step_n in 0..steps {
            for (entity1, closest_enemy, close_entities, closest_enemy_building) in
                entity_close.iter()
            {
                // Update entities
                entity1.borrow_mut().update(
                    closest_enemy.clone(),
                    closest_enemy_building.clone(),
                    // &mut self.projectile_handler,
                    step_n,
                    step_delta,
                    &mut event_handler,
                );

                // Entities push each other (Other team)
                for entity in close_entities {
                    if entity.borrow().get_id() == entity1.borrow().get_id() {
                        continue;
                    }
                    let is_same_team = entity.borrow().get_team() == entity1.borrow().get_team();
                    if !is_same_team {
                        continue;
                    }
                    let other_position = entity.borrow().get_position();
                    let other_radius = entity.borrow().get_radius();
                    entity1.borrow_mut().get_pushed(
                        other_position,
                        other_radius,
                        1.0,
                        is_same_team,
                    );
                }
                // Entities collide with ground
                entity1.borrow_mut().collide_with_ground(&self.ground, 1.0);

                // Entities push each other (Same team)
                for entity in close_entities {
                    if entity.borrow().get_id() == entity1.borrow().get_id() {
                        continue;
                    }
                    let is_same_team = entity.borrow().get_team() == entity1.borrow().get_team();
                    if is_same_team {
                        continue;
                    }
                    let other_position = entity.borrow().get_position();
                    let other_radius = entity.borrow().get_radius();
                    entity1.borrow_mut().get_pushed(
                        other_position,
                        other_radius,
                        1.0,
                        is_same_team,
                    );
                }

                // Flip updated position of each entity (should be done last after each move)
                entity1.borrow_mut().flip_position();
            }
        }

        // Update buildings
        self.building_container.update_buildings(&mut event_handler);

        while let Some(event) = event_handler.events.pop() {
            match event {
                Event::AddRangedProjectile { start, end, team } => self
                    .projectile_handler
                    .add_ranged_projectile(start, end, team),
                Event::AddMeleeProjectile { end, team } => {
                    self.projectile_handler.add_meelee_projectile(end, team)
                }
                Event::RequestGatherPath {
                    entity_id,
                    going_towards_resource,
                    resource_position,
                } => {
                    let entity_ref = self.entity_container.get_by_id(entity_id);
                    let entity_position: Vec2i = entity_ref.borrow().get_position().as_vec2i();
                    let positions: HashSet<Vec2i> =
                        [entity_position.clone()].iter().cloned().collect();
                    let entity_team = entity_ref.borrow().get_team();

                    if going_towards_resource {
                        let path = self.path_finder.find_path(
                            &self.ground,
                            PathGoal::Rect {
                                pos: resource_position,
                                size: Vec2i::new(1, 1),
                            },
                            &positions,
                        );
                        entity_ref.borrow_mut().update_path(path.clone());
                        self.debug_path = path.clone();
                    } else {
                        // TODO: Can not get the closest building like this. Must use path finding instead
                        if let Some(closest_building) =
                            self.building_container.get_closest_building(
                                // let closest_building: &Building = self.building_container.get_closest_building(
                                &entity_position,
                                Some(entity_team),
                                None,
                                9999999.9,
                            )
                        {
                            let path = self.path_finder.find_path(
                                &self.ground,
                                PathGoal::Rect {
                                    pos: closest_building.borrow().get_position(),
                                    size: Vec2i::new(
                                        closest_building.borrow().get_width(),
                                        closest_building.borrow().get_height(),
                                    ),
                                },
                                &positions,
                            );
                            entity_ref.borrow_mut().update_path(
                                path.clone(),
                                // Some(GatherGoalBuilding::new(
                                //     closest_building.get_position(),
                                //     Vec2i::new(
                                //         closest_building.get_width(),
                                //         closest_building.get_height(),
                                //     ),
                                // )),
                            );
                            self.debug_path = path.clone();
                        } else {
                            println!("No path to the building (no building found)");
                        }
                    }
                }
                Event::IncrementResources { team, amounts } => {
                    let team = self
                        .teams
                        .iter_mut()
                        .filter(|t| t.get_id() == team)
                        .next()
                        .unwrap();
                    team.increment_resources(&amounts);
                }
                Event::SpawnEntity {
                    entity_type,
                    building_id,
                    team,
                } => {
                    if let Some(building) = self.building_container.get_building_by_id(building_id)
                    {
                        println!("Spwaning an entity");
                        let new_entity = Entity::new_params(
                            building.borrow_mut().get_spawn_position(),
                            team,
                            entity_type,
                        );
                        let new_entity_id = new_entity.get_id();
                        self.entity_container.spawn_entity(new_entity);
                        if let Some(building_command_pos) =
                            building.borrow().get_spawn_command_position()
                        {
                            println!("Spawning entity: Commanding to move to position");
                            self.command_entities_move(
                                &vec![new_entity_id],
                                &building_command_pos,
                                false,
                            );
                        };
                    } else {
                        println!("Building that does not exists is spawning an entity");
                    }
                }
                Event::RequestRePath { entity_id } => {
                    let entity_ref = self.entity_container.get_by_id(entity_id);
                    let mut entity = entity_ref.borrow_mut();

                    println!("Rep pathing request handler");
                    entity.refresh_path(&mut self.path_finder, &self.ground);
                }
            }
        }

        // Update projectiles
        self.projectile_handler.progress_projectiles();
        for projectile in self.projectile_handler.get_impacting_projectiles() {
            if let Some(entity_hit) = self.entity_container.get_closest_entity(
                &projectile.get_position(),
                1.0, // TODO: Is this right??
                None,
                Some(projectile.get_team()),
            ) {
                entity_hit
                    .borrow_mut()
                    .health
                    .take_damage(projectile.get_damage());
            }
            if let Some(building_hit) = self.building_container.get_building_at(
                &projectile.get_position().as_vec2i(),
                None,
                Some(projectile.get_team()),
            ) {
                building_hit
                    .borrow_mut()
                    .health
                    .take_damage(projectile.get_damage());
            }
        }
        self.projectile_handler.remove_impacting_projectiles(); // Since impacting projectiles have been handled, remove them

        // Remove dead
        self.entity_container.remove_dead();
        self.building_container.remove_dead(&mut self.ground);
    }
}
