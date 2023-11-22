use crate::constants::{GROUND_HEIGHT, GROUND_WIDTH};
use crate::vec::Vec2f;

// Derive clone

#[derive(Clone, PartialEq)]
pub enum GroundType {
    Empty,
    Wall,
    Gold,
}

pub struct Ground {
    tiles: Vec<GroundType>,
    width: i32,
    height: i32,
}

impl Ground {
    pub fn new() -> Ground {
        let width: i32 = GROUND_WIDTH;
        let height: i32 = GROUND_HEIGHT;
        let mut tiles: Vec<GroundType> = Vec::new();
        for _ in 0..width * height {
            // Random change of being a wall
            if rand::random::<f32>() < 0.1 {
                tiles.push(GroundType::Wall);
            } else if rand::random::<f32>() < 0.02 {
                tiles.push(GroundType::Gold);
            } else {
                tiles.push(GroundType::Empty);
            }
            // tiles.push(GroundType::Empty);
        }
        let mut ground = Ground {
            tiles,
            width,
            height,
        };

        // Set corners to walls
        for x in 0..ground.width {
            ground.set_at(x, 0, GroundType::Wall);
            ground.set_at(x, ground.height - 1, GroundType::Wall);
        }
        for y in 0..ground.height {
            ground.set_at(0, y, GroundType::Wall);
            ground.set_at(ground.width - 1, y, GroundType::Wall);
        }

        ground
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn set_at(&mut self, x: i32, y: i32, ground_type: GroundType) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            println!("Ground set_at out of bounds");
            return;
        }
        self.tiles[(y * self.width + x) as usize] = ground_type;
    }

    pub fn get_at(&self, x: i32, y: i32) -> GroundType {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return GroundType::Wall;
        }
        self.tiles[(y * self.width + x) as usize].clone()
    }

    pub fn get_pos(&self, pos: &Vec2f) -> GroundType {
        self.get_at(pos.x as i32, pos.y as i32)
    }

    pub fn is_blocked(&self, pos: &Vec2f) -> bool {
        self.blocked_at(pos.x as i32, pos.y as i32)
    }

    pub fn blocked_at(&self, x: i32, y: i32) -> bool {
        match self.get_at(x, y) {
            GroundType::Empty => false,
            GroundType::Wall | GroundType::Gold => true,
        }
    }

    pub fn nearest_unblocked(&self, pos: &Vec2f) -> Option<Vec2f> {
        let start_x = pos.x as i32;
        let start_y = pos.y as i32;
        for range in 1..100 {
            for x in start_x - range..start_x + range {
                for y in start_y - range..start_y + range {
                    if !self.blocked_at(x, y) {
                        return Some(Vec2f::new(x as f32 + 0.5, y as f32 + 0.5));
                    }
                }
            }
        }
        None

        // if !self.is_blocked(&pos) {
        //     return Some(pos.clone());
        // }
        //
        // let mut nearest = pos.clone();
        // let mut distance = 0.0;
        // loop {
        //     distance += 1.0;
        //
        //     if distance > 100.0 {
        //         return None;
        //     }
        //
        //     nearest = pos + &Vec2f::new(distance, 0.0);
        //     if !self.is_blocked(&nearest) {
        //         return Some(nearest);
        //     }
        //     nearest = pos + &Vec2f::new(-distance, 0.0);
        //     if !self.is_blocked(&nearest) {
        //         return Some(nearest);
        //     }
        //     nearest = pos + &Vec2f::new(0.0, distance);
        //     if !self.is_blocked(&nearest) {
        //         return Some(nearest);
        //     }
        //     nearest = pos + &Vec2f::new(0.0, -distance);
        //     if !self.is_blocked(&nearest) {
        //         return Some(nearest);
        //     }
        // }
    }

    pub fn generate_goals(&self, center: &Vec2f, number: i32) -> Vec<Vec2f> {
        let mut goals: Vec<Vec2f> = Vec::new();

        // Generate points around center in a hex pattern
        if !self.blocked_at(center.x as i32, center.y as i32) {
            goals.push(center.clone());
        }
        // goals.push(center.clone());

        for radius in 1..100 {
            for i in 0..radius * 6 {
                let angle = i as f32 / (radius * 6) as f32 * 2.0 * std::f32::consts::PI;
                let x = center.x + angle.cos() * radius as f32;
                let y = center.y + angle.sin() * radius as f32;

                if !self.blocked_at(x as i32, y as i32) {
                    goals.push(Vec2f::new(x, y));
                }

                if goals.len() >= number as usize {
                    return goals;
                }
            }
        }

        goals
    }
}
