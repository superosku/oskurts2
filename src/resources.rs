pub struct Resources {
    pub gold: i32,
}

impl Resources {
    pub fn new_empty() -> Resources {
        Resources { gold: 0 }
    }

    pub fn new(gold: i32) -> Resources {
        Resources { gold }
    }

    pub fn can_afford(&self, other: &Resources) -> bool {
        self.gold >= other.gold
    }

    pub fn decrement(&mut self, cost: &Resources) {
        self.gold -= cost.gold;
    }

    pub fn increment(&mut self, amount: &Resources) {
        self.gold += amount.gold;
    }
}
