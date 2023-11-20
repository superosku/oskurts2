use crate::event_handler::{Event, EventHandler};
use crate::ground::Ground;
use crate::path_finder::{Path, PathFinder, PathGoal};
use crate::resources::Resources;
use crate::vec::{Vec2f, Vec2i};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum EntityType {
    Melee,
    Ranged,
    Worker,
}

#[derive(Clone)]
pub struct Goal {
    // position: Vec2f,
    group_size: f32,
    path: Rc<RefCell<Path>>,
}

impl Debug for Goal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Goal")
    }
}

// #[derive(Clone)]
// pub struct GatherGoalBuilding {
//     building_position: Vec2i,
//     building_size: Vec2i,
// }
//
// impl GatherGoalBuilding {
//     pub fn new(building_position: Vec2i, building_size: Vec2i) -> GatherGoalBuilding {
//         GatherGoalBuilding {
//             building_position,
//             building_size,
//         }
//     }
// }

#[derive(Clone)]
pub struct GatherGoal {
    resource_position: Vec2i,
    // building: Option<GatherGoalBuilding>,
    resource_type: u8,
    going_towards_resource: bool,
    counter: i32,
    path: Option<Rc<RefCell<Path>>>,
}

impl Debug for GatherGoal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GatherGoal")
    }
}

#[derive(Clone, Debug)]
pub enum EntityAction {
    Move(Goal),
    Attack(Goal),
    Idle,
    Gather(GatherGoal),
    Hold,
}

impl EntityAction {
    pub fn get_path_goal(&self) -> Option<PathGoal> {
        let path_ref = match self {
            EntityAction::Move(ref goal) | EntityAction::Attack(ref goal) => &goal.path,
            EntityAction::Gather(ref goal) => {
                if let Some(path) = &goal.path {
                    path
                } else {
                    return None;
                }
            }
            _ => return None,
        };
        Some(path_ref.borrow().goal.clone())
    }
}

pub struct Entity {
    position: Vec2f,
    next_position: Vec2f,
    action: EntityAction,
    speed: f32,
    id: usize,
    radius: f32,
    team: u8,
    projectile_cooldown: i32,
    health: i32,
    max_health: i32,
    entity_type: EntityType,
}

impl Clone for Entity {
    fn clone(&self) -> Self {
        panic!("Clone not implemented for Entity");
    }
}

impl Entity {
    pub fn new_params(position: Vec2f, team: u8, entity_type: EntityType) -> Entity {
        let mut entity = Entity::new(position);

        let radius = match entity_type {
            EntityType::Melee => 0.35,
            EntityType::Ranged => 0.4,
            EntityType::Worker => 0.25,
        };

        entity.team = team;
        entity.radius = radius;
        entity.entity_type = entity_type;

        entity
    }

    pub fn health_ratio(&self) -> f32 {
        self.health as f32 / self.max_health as f32
    }

    pub fn get_entity_type(&self) -> EntityType {
        self.entity_type.clone()
    }

    pub fn refresh_path(&mut self, path_finder: &mut PathFinder, ground: &Ground) {
        if let Some(path_goal) = self.action.get_path_goal() {
            let positions: HashSet<Vec2i> = [self.position.as_vec2i()].iter().cloned().collect();
            let new_path = path_finder.find_path(ground, path_goal, &positions);

            if let Some(path) = new_path {
                self.update_path(Some(path));
            } else {
                println!("Could not find path")
                // TODO: Should we set the path to null here to avoid infinite calls to path refreshing?
            }
        }
    }

    pub fn new(position: Vec2f) -> Entity {
        let random_id = rand::random::<usize>();
        // Radius should be random between 0.25 and 0.5
        let random_radius = rand::random::<f32>() / 4.0 + 0.25;
        // let random_radius = 0.5;
        let random_team = rand::random::<u8>() % 2;

        let random_entity_type = if rand::random::<f32>() < 0.5 {
            EntityType::Melee
        } else {
            EntityType::Ranged
        };

        Entity {
            position: position.clone(),
            next_position: position.clone(),
            action: EntityAction::Idle,
            speed: 0.05,
            id: random_id,
            radius: random_radius,
            team: random_team,
            projectile_cooldown: 0,
            health: 100,
            max_health: 100,
            entity_type: random_entity_type,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn get_team(&self) -> u8 {
        self.team
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    fn set_action(&mut self, action: EntityAction) {
        println!("set_action {:?}", action);
        self.action = action;
    }

    pub fn set_action_hold(&mut self) {
        self.set_action(EntityAction::Hold);
    }

    pub fn set_action_idle(&mut self) {
        self.set_action(EntityAction::Idle);
    }

    pub fn set_action_move(&mut self, path: Rc<RefCell<Path>>, goal: &Vec2f, goal_group_size: f32) {
        self.set_action(EntityAction::Move(Goal {
            path,
            // position: goal.clone(),
            group_size: goal_group_size,
        }));
    }

    pub fn set_action_attack(
        &mut self,
        path: Rc<RefCell<Path>>,
        goal: &Vec2f,
        goal_group_size: f32,
    ) {
        self.set_action(EntityAction::Attack(Goal {
            path,
            // position: goal.clone(),
            group_size: goal_group_size,
        }));
    }

    pub fn set_action_gather(
        &mut self,
        resource_position: Vec2i,
        resource_type: u8,
        path: Option<Rc<RefCell<Path>>>,
    ) {
        self.set_action(EntityAction::Gather(GatherGoal {
            resource_position,
            resource_type,
            going_towards_resource: true,
            counter: 0,
            path,
        }));
    }

    pub fn get_goal(&self) -> Option<Vec2f> {
        let path_goal = self.action.get_path_goal();
        // let path_ref = match self.action {
        //     EntityAction::Move (ref goal) |
        //     EntityAction::Attack (ref goal) => {
        //         &goal.path
        //     }
        //     EntityAction::Gather(ref goal) => {
        //         if let Some(path) = &goal.path {
        //             path
        //         } else {
        //             return None
        //         }
        //     },
        //     _ => {
        //         return None
        //     }
        // };

        Some(match path_goal {
            Some(PathGoal::Rect { pos, size }) => pos.as_vec2f() + size.as_vec2f() / 2.0,
            Some(PathGoal::Point { pos }) => pos.clone(),
            None => return None,
        })

        // Some(match &path_ref.borrow().goal {
        //     PathGoal::Rect {pos, size} => {
        //         pos.as_vec2f() + size.as_vec2f() / 2.0
        //     },
        //     PathGoal::Point {pos} => {
        //         pos.clone()
        //     }
        // })
    }

    pub fn update_path(
        &mut self,
        path: Option<Rc<RefCell<Path>>>,
        // building: Option<GatherGoalBuilding>,
    ) {
        match &mut self.action {
            EntityAction::Gather(ref mut goal) => {
                goal.path = path;
                // goal.building = building;
            }
            _ => {
                println!("update_path called on non gather action (NOT YET IMPLEMENTED)");
            }
        }
    }

    pub fn get_position(&self) -> Vec2f {
        self.position.clone()
    }

    pub fn get_pushed(
        &mut self,
        other_position: Vec2f,
        other_radius: f32,
        step_delta: f32,
        is_same_team: bool,
    ) {
        let delta = self.position.clone() - other_position;
        if delta.length() == 0.0 {
            println!("Delta length is 0.0");
            let random_value =
                (self.id & 18446744073709551615) as f32 / 18446744073709551615.0 / 10.0;
            self.next_position += Vec2f::new(random_value, random_value * 0.5);
            return;
        }

        let radius_sum = self.radius + other_radius;

        let delta_length = delta.length();
        if delta_length < radius_sum {
            let vector_away_from_other = delta.normalized();
            let overlap_amount = radius_sum - delta_length;
            if is_same_team {
                self.next_position += vector_away_from_other * overlap_amount * step_delta / 2.0;
            } else {
                let total_overlap = overlap_amount;
                let move_dist = self.position.clone() - self.next_position.clone();
                let total_move = move_dist.length();

                let move_amount = (total_overlap * step_delta).min(total_move);

                self.next_position += vector_away_from_other * move_amount;
            }
        }
    }

    pub fn flip_position(&mut self) {
        self.position = self.next_position.clone();
    }

    pub fn collide_with_ground(&mut self, ground: &Ground, step_delta: f32) {
        // Completely inside a block
        if ground.is_blocked(&self.next_position) {
            println!("Completely inside a block");
            match ground.nearest_unblocked(&self.next_position) {
                Some(nearest_unblocked) => {
                    self.next_position = nearest_unblocked;
                }
                None => {
                    println!("No nearest unblocked");
                }
            }
        }

        let radius = self.radius;

        // Partially inside a block directly side by side
        if ground.is_blocked(&(self.next_position.clone() + Vec2f::new(radius, 0.0))) {
            self.next_position.x = self.next_position.x.floor() + (1.0 - radius);
        }
        if ground.is_blocked(&(self.next_position.clone() + Vec2f::new(-radius, 0.0))) {
            self.next_position.x = self.next_position.x.ceil() - (1.0 - radius);
        }
        if ground.is_blocked(&(self.next_position.clone() + Vec2f::new(0.0, radius))) {
            self.next_position.y = self.next_position.y.floor() + (1.0 - radius);
        }
        if ground.is_blocked(&(self.next_position.clone() + Vec2f::new(0.0, -radius))) {
            self.next_position.y = self.next_position.y.ceil() - (1.0 - radius);
        }

        // Partially inside a block diagonally
        let floor_x = self.next_position.x.floor() as i32;
        let floor_x_diff = (self.next_position.x - floor_x as f32).abs();
        let floor_y = self.next_position.y.floor() as i32;
        let floor_y_diff = (self.next_position.y - floor_y as f32).abs();
        let ceil_x = self.next_position.x.ceil() as i32;
        let ceil_x_diff = (ceil_x as f32 - self.next_position.x).abs();
        let ceil_y = self.next_position.y.ceil() as i32;
        let ceil_y_diff = (ceil_y as f32 - self.next_position.y).abs();
        let thing = [
            (0, floor_x_diff + floor_y_diff),
            (1, floor_x_diff + ceil_y_diff),
            (2, ceil_x_diff + floor_y_diff),
            (3, ceil_x_diff + ceil_y_diff),
        ];
        let min = thing
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        let corner_match: i32 = min.0;

        if corner_match == 0 && ground.blocked_at(floor_x - 1, floor_y - 1) {
            let corner_pos = Vec2f::new(floor_x as f32, floor_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < radius {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = radius - delta_length;
                self.next_position += vector_away_from_other * overlap_amount * step_delta;
            }
        }
        if corner_match == 1 && ground.blocked_at(floor_x - 1, ceil_y) {
            let corner_pos = Vec2f::new(floor_x as f32, ceil_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < radius {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = radius - delta_length;
                self.next_position += vector_away_from_other * overlap_amount * step_delta;
            }
        }
        if corner_match == 2 && ground.blocked_at(ceil_x, floor_y - 1) {
            let corner_pos = Vec2f::new(ceil_x as f32, floor_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < radius {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = radius - delta_length;
                self.next_position += vector_away_from_other * overlap_amount * step_delta;
            }
        }
        if corner_match == 3 && ground.blocked_at(ceil_x, ceil_y) {
            let corner_pos = Vec2f::new(ceil_x as f32, ceil_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < radius {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = radius - delta_length;
                self.next_position += vector_away_from_other * overlap_amount * step_delta;
            }
        }
    }

    // // Helper for fn update
    // fn distance_to_goal(&self, goal: &Vec2f) -> f32 {
    //     let delta = goal.clone() - self.position.clone();
    //     delta.length()
    // }
    //
    // fn distance_to_big_block(&self, block: &Vec2i, block_size: &Vec2i) -> f32 {
    //     let y_diff = if self.position.y < block.y as f32 {
    //         block.y as f32 - self.position.y
    //     } else if self.position.y > (block.y + block_size.y) as f32 {
    //         self.position.y - (block.y + block_size.y) as f32
    //     } else {
    //         0.0
    //     };
    //
    //     let x_diff = if self.position.x < block.x as f32 {
    //         block.x as f32 - self.position.x
    //     } else if self.position.x > (block.x + block_size.x) as f32 {
    //         self.position.x - (block.x + block_size.x) as f32
    //     } else {
    //         0.0
    //     };
    //
    //     (x_diff * x_diff + y_diff * y_diff).sqrt()
    // }
    //
    // fn distance_to_path_goal(&self, path: &Path) -> f32 {
    //     match &path.goal {
    //         PathGoal::Point{pos} => self.distance_to_block(block),
    //         PathGoal::Rect{pos, size} => self.distance_to_goal(position),
    //     }
    // }

    // Helper for fn update
    // fn distance_to_block(&self, block: &Vec2i) -> f32 {
    //     self.distance_to_big_block(block, &Vec2i::new(1, 1))
    // }

    // Helper for fn update
    fn move_towards_goal(&mut self, goal: &Vec2f, step_delta: f32) {
        let delta = goal.clone() - self.position.clone();
        self.next_position += delta.normalized() * self.speed * step_delta;
    }

    fn move_towards_path(
        &mut self,
        path: Rc<RefCell<Path>>,
        step_delta: f32,
        event_handler: &mut EventHandler,
    ) {
        let path = path.borrow();

        match &path.goal {
            PathGoal::Rect { pos, size } => {
                // TODO: Should perhaps move towards the big block here?
                // Or is it enough to move towards the path finding arrows instead?
            }
            PathGoal::Point { pos } => {
                if path.distance_to_goal(&self.position) < 1.0 {
                    self.move_towards_goal(pos, step_delta);
                    return;
                }
            }
        };

        let mut directions: Vec<Vec2f> = Vec::new();
        for i in [
            (self.position.clone() + Vec2f::new(0.0, self.radius)).as_vec2i(),
            (self.position.clone() + Vec2f::new(0.0, -self.radius)).as_vec2i(),
            (self.position.clone() + Vec2f::new(self.radius, 0.0)).as_vec2i(),
            (self.position.clone() + Vec2f::new(-self.radius, 0.0)).as_vec2i(),
        ] {
            if let Some(direction) = path.get_direction(&i) {
                directions.push(direction);
            }
        }

        let mut had_direction = false;
        let mut avg_direction = Vec2f::new(0.0, 0.0);
        for direction in directions.iter() {
            avg_direction += direction.clone();
            had_direction = true;
        }
        if !had_direction {
            println!("Has falleng outside path, requesting a re path...");
            event_handler.add_event(Event::RequestRePath { entity_id: self.id });
            return;
        }

        avg_direction = avg_direction / directions.len() as f32;

        let asdf_goal = self.position.clone() + avg_direction * 10.0;

        self.move_towards_goal(&asdf_goal, step_delta);
    }

    // Helper for fn update
    fn interact_with_closest_enemy(
        &mut self,
        closest_enemy: &Rc<RefCell<Entity>>,
        // projectile_handler: &mut ProjectileHandler,
        step_n: i32,
        step_delta: f32,
        can_move: bool,
        event_handler: &mut EventHandler,
    ) {
        let enemy_position = closest_enemy.borrow().position.clone();
        let delta = enemy_position.clone() - self.position.clone();
        let delta_length = delta.length();
        let combined_length = self.radius + closest_enemy.borrow().radius;

        let min_range = match self.entity_type {
            EntityType::Ranged => 5.0,
            EntityType::Melee => combined_length + 0.1,
            EntityType::Worker => {
                // Workers do not interact with enemies
                // TODO: Should it run away?
                return;
            }
        };

        if delta_length > min_range {
            if can_move {
                self.move_towards_goal(&enemy_position, step_delta);
            }
        } else {
            if step_n == 0 {
                if self.projectile_cooldown == 0 {
                    match self.entity_type {
                        EntityType::Ranged => {
                            event_handler.add_event(Event::AddRangedProjectile {
                                start: self.position.clone(),
                                end: closest_enemy.borrow().position.clone(),
                                team: self.team,
                            });
                        }
                        EntityType::Melee => {
                            event_handler.add_event(Event::AddMeleeProjectile {
                                end: closest_enemy.borrow().position.clone(),
                                team: self.team,
                            });
                        }
                        EntityType::Worker => {}
                    }
                    self.projectile_cooldown = 100;
                } else {
                    self.projectile_cooldown -= 1;
                }
            }
        }
    }

    pub fn handle_gather(
        &mut self,
        goal: &mut GatherGoal,
        step_n: i32,
        step_delta: f32,
        event_handler: &mut EventHandler,
    ) {
        if let Some(path_ref) = &goal.path {
            let path = path_ref.borrow();
            if goal.going_towards_resource {
                if path.distance_to_goal(&self.position) < self.radius + 0.1 {
                    // if self.distance_to_block(&goal.resource_position.as_vec2i()) < self.radius + 0.1 {
                    if step_n == 0 {
                        goal.counter += 1;
                        if goal.counter > 200 {
                            // TODO: Decrement map resource here
                            goal.counter = 0;
                            goal.going_towards_resource = false;

                            // println!("Requesting new path");
                            event_handler.add_event(Event::RequestGatherPath {
                                entity_id: self.id,
                                going_towards_resource: false,
                                resource_position: goal.resource_position.clone(),
                            })
                        }
                    }
                } else {
                    if let Some(path) = &goal.path {
                        // println!("Moving towards path 1");
                        self.move_towards_path(path.clone(), step_delta, event_handler);
                    } else {
                        // I do not think this should happen :thinking:
                        println!("No path found 1");
                        // self.move_towards_goal(&goal.resource_position.clone(), step_delta);
                    }
                }
            } else {
                if path.distance_to_goal(&self.position) < self.radius + 0.1 {
                    // if let Some(building_data) = goal.building.clone() {
                    //     if self.distance_to_big_block(
                    //         &building_data.building_position,
                    //         &building_data.building_size,
                    //     ) < self.radius + 0.1
                    //     {
                    if step_n == 0 {
                        // TODO: This is a dropoff point
                        // TODO: Increment resource here
                        goal.counter = 0;
                        goal.going_towards_resource = true;

                        event_handler.add_event(Event::RequestGatherPath {
                            entity_id: self.id,
                            going_towards_resource: true,
                            resource_position: goal.resource_position.clone(),
                        });
                        event_handler.add_event(Event::IncrementResources {
                            team: self.team,
                            amounts: Resources::new(20),
                        });
                    }
                } else {
                    if let Some(path) = &goal.path {
                        // self.move_towards_path(path.clone(), &Vec2f::new(-1.0, -1.0), step_delta);
                        // TODO: What was this -1.0 hack here for?
                        self.move_towards_path(path.clone(), step_delta, event_handler);
                    } else {
                        // I do not think this should happen :thinking:
                        println!("No path found 2");
                        // self.move_towards_goal(&goal.resource_position.clone(), step_delta);
                    }
                }
                // } else {
                //     println!("HMM :(");
                // }
            }
        }
    }

    pub fn update(
        &mut self,
        closest_enemy: Option<Rc<RefCell<Entity>>>,
        step_n: i32,
        step_delta: f32,
        event_handler: &mut EventHandler,
    ) {
        // TODO: This is a hack against multiple mutable borrows
        let mut cloned_action = self.action.clone();

        match &mut cloned_action {
            EntityAction::Idle => {
                if let Some(closest_enemy) = closest_enemy {
                    self.interact_with_closest_enemy(
                        &closest_enemy,
                        step_n,
                        step_delta,
                        true,
                        event_handler,
                    );
                } else {
                }
            }
            EntityAction::Move(goal) => {
                if goal.path.borrow().distance_to_goal(&self.position)
                    < (goal.group_size.sqrt() - (self.radius * 2.0)).max(0.1)
                {
                    // println!(
                    //     "We are done with move {} {} {}",
                    //     goal.path.borrow().distance_to_goal(&self.position),
                    //     goal.group_size.sqrt(),
                    //     self.radius,
                    // );
                    // if self.distance_to_goal(&goal.position) < 1.0 * (goal.group_size).sqrt() / 1.5 {
                    cloned_action = EntityAction::Idle;
                } else {
                    // self.move_towards_path(goal.path.clone(), &goal.position, step_delta);
                    self.move_towards_path(goal.path.clone(), step_delta, event_handler);
                }
            }
            EntityAction::Attack(goal) => {
                if goal.path.borrow().distance_to_goal(&self.position)
                    < (goal.group_size.sqrt() - (self.radius * 2.0)).max(0.1)
                {
                    println!("We are done with attack move");
                    // if self.distance_to_goal(&goal.position) < 1.0 * (goal.group_size).sqrt() / 2.0 {
                    cloned_action = EntityAction::Idle;
                } else {
                    if let Some(closest_enemy) = closest_enemy {
                        self.interact_with_closest_enemy(
                            &closest_enemy,
                            step_n,
                            step_delta,
                            true,
                            event_handler,
                        );
                    } else {
                        // self.move_towards_path(goal.path.clone(), &goal.position, step_delta);
                        self.move_towards_path(goal.path.clone(), step_delta, event_handler);
                    }
                }
            }
            EntityAction::Gather(goal) => {
                self.handle_gather(goal, step_n, step_delta, event_handler);
            }
            EntityAction::Hold => {
                if let Some(closest_enemy) = closest_enemy {
                    self.interact_with_closest_enemy(
                        &closest_enemy,
                        step_n,
                        step_delta,
                        false,
                        event_handler,
                    );
                } else {
                }
            }
        }

        self.action = cloned_action;
    }
}
