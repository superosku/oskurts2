pub struct Health {
    health: i32,
    max_health: i32,
}

impl Health {
    pub fn new(max_health: i32) -> Health {
        Health {
            health: max_health,
            max_health,
        }
    }

    pub fn new_with_health(health: i32, max_health: i32) -> Health {
        Health { health, max_health }
    }

    pub fn increment(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn health_ratio(&self) -> f32 {
        self.health as f32 / self.max_health as f32
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}
