use crate::game_thing::GameThing;
use crate::ground::Ground;
use crate::vec::Vec2f;

pub struct Entity {
    position: Vec2f,
    next_position: Vec2f,
    goal: Option<Vec2f>,
    speed: f32,
}

impl Entity {
    pub fn new(position: Vec2f) -> Entity {
        Entity {
            position: position.clone(),
            next_position: position.clone(),
            goal: Some(Vec2f::new(10.0, 10.0)),
            speed: 0.02,
        }
    }

    pub fn get_position(&self) -> Vec2f {
        self.position.clone()
    }

    pub fn get_pushed(&mut self, other_position: Vec2f) {
        let delta = self.position.clone() - other_position;
        let delta_length = delta.length();
        if delta_length < 1.0 {
            let vector_away_from_other = delta.normalized();
            let overlap_amount = 1.0 - delta_length;
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

        // Partially inside a block directly side by side
        if ground.is_blocked(&(self.next_position.clone() + Vec2f::new(0.5, 0.0))) {
            self.next_position.x = self.next_position.x.floor() + 0.5;
        }
        if ground.is_blocked(&(self.next_position.clone() + Vec2f::new(-0.5, 0.0))) {
            self.next_position.x = self.next_position.x.ceil() - 0.5;
        }
        if ground.is_blocked(&(self.next_position.clone() + Vec2f::new(0.0, 0.5))) {
            self.next_position.y = self.next_position.y.floor() + 0.5;
        }
        if ground.is_blocked(&(self.next_position.clone() + Vec2f::new(0.0, -0.5))) {
            self.next_position.y = self.next_position.y.ceil() - 0.5;
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
            if delta_length < 0.5 {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = 0.5 - delta_length;
                self.next_position += vector_away_from_other * overlap_amount;
            }
        }
        if corner_match == 1 && ground.blocked_at(floor_x - 1, ceil_y) {
            let corner_pos = Vec2f::new(floor_x as f32, ceil_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < 0.5 {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = 0.5 - delta_length;
                self.next_position += vector_away_from_other * overlap_amount;
            }
        }
        if corner_match == 2 && ground.blocked_at(ceil_x, floor_y - 1) {
            let corner_pos = Vec2f::new(ceil_x as f32, floor_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < 0.5 {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = 0.5 - delta_length;
                self.next_position += vector_away_from_other * overlap_amount;
            }
        }
        if corner_match == 3 && ground.blocked_at(ceil_x, ceil_y) {
            let corner_pos = Vec2f::new(ceil_x as f32, ceil_y as f32);
            let delta = self.next_position.clone() - corner_pos;
            let delta_length = delta.length();
            if delta_length < 0.5 {
                let vector_away_from_other = delta.normalized();
                let overlap_amount = 0.5 - delta_length;
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
                // if delta_length < 0.1 {
                //     self.goal = None;
                // } else {
                self.next_position += delta.normalized() * self.speed;
                // }
            }
            None => {
                println!("No goal set");
            }
        }
    }
}
