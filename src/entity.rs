use crate::game_thing::GameThing;
use crate::ground::Ground;
use crate::vec::Vec2f;

pub struct Entity {
    position: Vec2f,
    next_position: Vec2f,
    goal: Option<Vec2f>,
    goal_group_size: i32,
    speed: f32,
    id: usize,
    radius: f32,
}

impl Entity {
    pub fn new(position: Vec2f) -> Entity {
        let random_id = rand::random::<usize>();
        // Radius should be random between 0.25 and 0.5
        let random_radius = rand::random::<f32>() / 4.0 + 0.25;
        // let random_radius = 0.5;

        Entity {
            position: position.clone(),
            next_position: position.clone(),
            // goal: Some(Vec2f::new(10.0, 10.0)),
            goal: None,
            speed: 0.05,
            id: random_id,
            goal_group_size: 1,
            radius: random_radius,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn set_goal(&mut self, goal: &Vec2f, goal_group_size: i32) {
        self.goal = Some(goal.clone());
        self.goal_group_size = goal_group_size;
    }

    pub fn get_position(&self) -> Vec2f {
        self.position.clone()
    }

    pub fn get_goal(&self) -> Option<Vec2f> {
        self.goal.clone()
    }

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
}

impl GameThing for Entity {
    fn update(&mut self) {
        match &self.goal {
            Some(goal) => {
                let delta = goal.clone() - self.position.clone();
                let delta_length = delta.length();
                if delta_length < (1.0 * (self.goal_group_size as f32).sqrt() / 2.0 - 0.9).max(0.1)
                {
                    self.goal = None;
                } else {
                    self.next_position += delta.normalized() * self.speed;
                }
                // let speed = (self.next_position.clone() - self.position.clone()).length();
                // if speed < self.speed / 2.0 {
                //     println!("Going half the speed");
                // }
            }
            None => {
                // println!("No goal set");
            }
        }
    }
}
