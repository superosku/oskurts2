use crate::vec::{Vec2f, Vec2i};

pub struct Building {
    position: Vec2i,
    width: i32,
    height: i32,
    id: usize,
    team: u8,
}

impl Building {
    pub fn new(position: Vec2i, width: i32, height: i32, team: u8) -> Building {
        let random_id = rand::random::<usize>();
        Building {
            position,
            width,
            height,
            id: random_id,
            team,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_team(&self) -> u8 {
        self.team
    }

    pub fn get_spawn_position(&self) -> Vec2f {
        Vec2f::new(self.position.x as f32 - 0.5, self.position.y as f32 - 0.5)
    }

    pub fn get_position(&self) -> Vec2i {
        self.position.clone()
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}
