use crate::ground::Ground;
use crate::projectile_handler::ProjectileHandler;
use crate::vec::{Vec2f, Vec2i};
use std::cell::RefCell;
use std::rc::Rc;

pub enum EntityType {
    Meelee,
    Ranged,
}

pub struct Goal {
    position: Vec2f,
    group_size: i32,
}

pub struct GatherGoal {
    resource_position: Vec2f,
    building_position: Vec2f,
    resource_type: u8,
}

pub enum EntityAction {
    Move(Goal),
    Attack(Goal),
    Idle,
    Gather(GatherGoal),
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

impl Entity {
    pub fn new_params(position: Vec2f, team: u8, radius: f32, entity_type: EntityType) -> Entity {
        let mut entity = Entity::new(position);

        entity.team = team;
        entity.radius = radius;
        entity.entity_type = entity_type;

        entity
    }

    pub fn new(position: Vec2f) -> Entity {
        let random_id = rand::random::<usize>();
        // Radius should be random between 0.25 and 0.5
        let random_radius = rand::random::<f32>() / 4.0 + 0.25;
        // let random_radius = 0.5;
        let random_team = rand::random::<u8>() % 2;

        let random_entity_type = if rand::random::<f32>() < 0.5 {
            EntityType::Meelee
        } else {
            EntityType::Ranged
        };

        Entity {
            position: position.clone(),
            next_position: position.clone(),
            // goal: Some(Vec2f::new(10.0, 10.0)),
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
        println!("Taking damage! {}", self.health);
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

    pub fn set_goal(&mut self, goal: &Vec2f, goal_group_size: i32) {
        // TODO: Check if should be gather instead
        // TODO: Allow setting attack action
        self.action = EntityAction::Move(Goal {
            position: goal.clone(),
            group_size: goal_group_size,
        });
        // self.goal = Some(goal.clone());
        // self.goal_group_size = goal_group_size;
    }

    pub fn get_goal(&self) -> Option<Vec2f> {
        match self.action {
            EntityAction::Move(ref goal) => Some(goal.position.clone()),
            EntityAction::Attack(ref goal) => Some(goal.position.clone()),
            EntityAction::Gather(ref gather_goal) => {
                // TODO: Return building position instead of resource position if moving towards
                // the building
                Some(gather_goal.resource_position.clone())
            }
            _ => None,
        }
    }

    pub fn get_position(&self) -> Vec2f {
        self.position.clone()
    }
    //
    // pub fn get_goal(&self) -> Option<Vec2f> {
    //     self.goal.clone()
    // }

    pub fn get_pushed(&mut self, other_position: Vec2f, other_radius: f32) {
        let delta = self.position.clone() - other_position;
        if delta.length() == 0.0 {
            println!("Delta length is 0.0");
            let random_value =
                (self.id & 18446744073709551615) as f32 / 18446744073709551615.0 / 10.0;
            self.next_position += Vec2f::new(random_value, 0.0);
            return;
        }

        let radius_sum = self.radius + other_radius;

        let delta_length = delta.length();
        if delta_length < radius_sum {
            let vector_away_from_other = delta.normalized();
            let overlap_amount = radius_sum - delta_length;
            self.next_position += vector_away_from_other * overlap_amount / 2.0;
        }
    }

    pub fn flip_position(&mut self) {
        self.position = self.next_position.clone();
    }

    pub fn collide_with_ground(&mut self, ground: &Ground) {
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
        // println!(
        //     "floor_x: {}, floor_x_diff: {}, floor_y: {}, floor_y_diff: {}, ceil_x: {}, ceil_x_diff: {}, ceil_y: {}, ceil_y_diff: {}",
        //     floor_x, floor_x_diff, floor_y, floor_y_diff, ceil_x, ceil_x_diff, ceil_y, ceil_y_diff);
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
                self.next_position += vector_away_from_other * overlap_amount;
            }
        }
        if corner_match == 1 && ground.blocked_at(floor_x - 1, ceil_y) {
            let corner_pos = Vec2f::new(floor_x as f32, ceil_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < radius {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = radius - delta_length;
                self.next_position += vector_away_from_other * overlap_amount;
            }
        }
        if corner_match == 2 && ground.blocked_at(ceil_x, floor_y - 1) {
            let corner_pos = Vec2f::new(ceil_x as f32, floor_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < radius {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = radius - delta_length;
                self.next_position += vector_away_from_other * overlap_amount;
            }
        }
        if corner_match == 3 && ground.blocked_at(ceil_x, ceil_y) {
            let corner_pos = Vec2f::new(ceil_x as f32, ceil_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < radius {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = radius - delta_length;
                self.next_position += vector_away_from_other * overlap_amount;
            }
        }
    }

    // Helper for fn update
    fn distance_to_goal(&self, goal: &Vec2f) -> f32 {
        let delta = goal.clone() - self.position.clone();
        delta.length()
    }

    // Helper for fn update
    fn distance_to_block(&self, block: &Vec2i) -> f32 {
        // Returns a distance to block. For an example (10,10) from (10.5, 10.5) is 0.5
        // And (10,10) from (10.5, 11.5) is 0.5

        let y_diff = if self.position.y < block.y as f32 {
            block.y as f32 - self.position.y
        } else if self.position.y > block.y as f32 + 1.0 {
            self.position.y - (block.y as f32 + 1.0)
        } else {
            0.0
        };

        let x_diff = if self.position.x < block.x as f32 {
            block.x as f32 - self.position.x
        } else if self.position.x > block.x as f32 + 1.0 {
            self.position.x - (block.x as f32 + 1.0)
        } else {
            0.0
        };

        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }

    // Helper for fn update
    fn move_towards_goal(&mut self, goal: &Vec2f) {
        let delta = goal.clone() - self.position.clone();
        self.next_position += delta.normalized() * self.speed;
    }

    // Helper for fn update
    fn interact_with_closest_enemy(
        &mut self,
        closest_enemy: &Rc<RefCell<Entity>>,
        projectile_handler: &mut ProjectileHandler,
    ) {
        let delta = closest_enemy.borrow().position.clone() - self.position.clone();
        let delta_length = delta.length();
        let combined_length = self.radius + closest_enemy.borrow().radius;

        let min_range = match self.entity_type {
            EntityType::Ranged => 5.0,
            EntityType::Meelee => combined_length + 0.1,
        };

        if delta_length > min_range {
            self.next_position += delta.normalized() * self.speed;
        } else {
            if self.projectile_cooldown == 0 {
                match self.entity_type {
                    EntityType::Ranged => {
                        projectile_handler.add_ranged_projectile(
                            self.position.clone(),
                            closest_enemy.borrow().position.clone(),
                        );
                    }
                    EntityType::Meelee => {
                        projectile_handler
                            .add_meelee_projectile(closest_enemy.borrow().position.clone());
                    }
                }
                self.projectile_cooldown = 100;
            } else {
                self.projectile_cooldown -= 1;
            }
            // TODO: Put down a projectile
            // TODO: How about cooldown
        }
    }

    pub fn update(
        &mut self,
        closest_enemy: Option<&Rc<RefCell<Entity>>>,
        projectile_handler: &mut ProjectileHandler,
    ) {
        match &self.action {
            EntityAction::Idle => {
                if let Some(closest_enemy) = closest_enemy {
                    self.interact_with_closest_enemy(closest_enemy, projectile_handler);
                }
            }
            EntityAction::Move(goal) => {
                if self.distance_to_goal(&goal.position)
                    < 1.0 * (goal.group_size as f32).sqrt() / 2.0
                {
                    self.action = EntityAction::Idle;
                } else {
                    self.move_towards_goal(&goal.position.clone());
                }
            }
            EntityAction::Attack(goal) => {
                if self.distance_to_goal(&goal.position)
                    < 1.0 * (goal.group_size as f32).sqrt() / 2.0
                {
                    self.action = EntityAction::Idle;
                } else {
                    if let Some(closest_enemy) = closest_enemy {
                        self.interact_with_closest_enemy(closest_enemy, projectile_handler);
                    } else {
                        self.move_towards_goal(&goal.position.clone());
                    }
                }
            }
            EntityAction::Gather(goal) => {
                if self.distance_to_block(&goal.resource_position.as_vec2i()) < self.radius + 0.1 {
                    self.action = EntityAction::Idle;
                    // TODO: Do not set action to idle here but instead start gathering resources
                    // TODO: And when done flip a switch to go back to the building etc..
                } else {
                    self.move_towards_goal(&goal.resource_position.clone());
                }
            }
        }
    }
}
