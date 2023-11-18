use crate::projectile::Projectile;
use crate::vec::Vec2f;
use std::slice::Iter;

pub struct ProjectileHandler {
    projectiles: Vec<Projectile>,
}

impl ProjectileHandler {
    pub fn new() -> ProjectileHandler {
        ProjectileHandler {
            projectiles: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<Projectile> {
        self.projectiles.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Projectile> {
        self.projectiles.iter_mut()
    }

    pub fn progress_projectiles(&mut self) {
        for projectile in self.projectiles.iter_mut() {
            projectile.progress();
        }
    }

    pub fn get_impacting_projectiles(&self) -> Vec<&Projectile> {
        let mut impacting_projectiles: Vec<&Projectile> = Vec::new();
        for projectile in self.projectiles.iter() {
            if projectile.ready_to_impact() {
                impacting_projectiles.push(projectile);
            }
            // let projectile_position = projectile.get_position();
            // let distance = projectile_position.distance_to(&position);
            // if distance < radius {
            //     impacting_projectiles.push(projectile.clone());
            // }
        }
        impacting_projectiles
    }

    pub fn remove_impacting_projectiles(&mut self) {
        self.projectiles
            .retain(|projectile| !projectile.ready_to_impact())
    }

    pub fn add_meelee_projectile(&mut self, position: Vec2f, team: u8) {
        let projectile = Projectile::new(position, None, 9, 0.0, team);
        self.projectiles.push(projectile);
    }

    pub fn add_ranged_projectile(&mut self, position: Vec2f, goal: Vec2f, team: u8) {
        let projectile = Projectile::new(position, Some(goal), 9, 0.1, team);
        self.projectiles.push(projectile);
    }
}
