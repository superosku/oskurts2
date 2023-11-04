use crate::game_thing::GameThing;
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
            goal: Some(Vec2f::new(0.0, 0.0)),
            speed: 0.05,
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
