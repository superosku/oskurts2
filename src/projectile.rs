use crate::vec::Vec2f;

pub struct Projectile {
    position: Vec2f,
    goal: Option<Vec2f>,
    damage: i32,
    speed: f32,
    team: u8,
}

impl Projectile {
    pub fn new(
        position: Vec2f,
        goal: Option<Vec2f>,
        damage: i32,
        speed: f32,
        team: u8,
    ) -> Projectile {
        Projectile {
            position,
            goal,
            damage,
            speed,
            team,
        }
    }

    pub fn get_team(&self) -> u8 {
        self.team
    }

    pub fn get_position(&self) -> Vec2f {
        self.position.clone()
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
    }

    pub fn progress(&mut self) {
        if let Some(goal) = &self.goal {
            let direction = goal.clone() - self.position.clone();
            let distance = direction.length();
            let direction = direction.normalized();
            let distance_to_travel = self.speed;
            if distance < distance_to_travel {
                self.position = goal.clone();
                self.goal = None
            } else {
                self.position += direction * distance_to_travel;
            }
        }
    }

    pub fn ready_to_impact(&self) -> bool {
        match self.goal {
            Some(_) => false,
            None => true,
        }
    }
}
