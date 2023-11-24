use crate::resources::Resources;

pub struct Team {
    resources: Resources,
    team_id: u8,
}

impl Team {
    pub fn new(team_id: u8) -> Team {
        Team {
            resources: Resources::new(1000),
            team_id,
        }
    }

    pub fn get_id(&self) -> u8 {
        self.team_id
    }

    pub fn get_resources(&self) -> &Resources {
        &self.resources
    }

    pub fn increment_resources(&mut self, amount: &Resources) {
        self.resources.increment(amount);
    }

    pub fn decrement_resources(&mut self, amount: &Resources) {
        self.resources.decrement(amount);
    }
}
