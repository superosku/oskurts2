use crate::building::Building;
use crate::entity::EntityType;
use crate::event_handler::EventHandler;
use crate::ground::{Ground, GroundType};
use crate::vec::{Vec2f, Vec2i};
use std::cell::RefCell;
use std::rc::Rc;

pub struct BuildingContainer {
    buildings: Vec<Rc<RefCell<Building>>>,
}

impl BuildingContainer {
    pub fn new() -> BuildingContainer {
        BuildingContainer {
            buildings: Vec::new(),
        }
    }

    pub fn add_building(&mut self, building: Building, ground: &mut Ground) {
        for x in building.get_position().x..building.get_position().x + building.get_width() {
            for y in building.get_position().y..building.get_position().y + building.get_height() {
                ground.set_at(x, y, GroundType::Wall);
            }
        }
        // TODO: Set ground tiles to occupied
        self.buildings.push(Rc::new(RefCell::new(building)));
    }

    // TODO: Remove building (set ground tiles to unoccupied)

    pub fn get_buildings(&self) -> &Vec<Rc<RefCell<Building>>> {
        &self.buildings
    }

    pub fn update_buildings(&mut self, event_handler: &mut EventHandler) {
        for building in self.buildings.iter_mut() {
            building.borrow_mut().update(event_handler);
        }
    }

    pub fn add_to_building_spawn_queue(&mut self, building_id: usize, entity_type: EntityType) {
        for building in self.buildings.iter_mut() {
            let this_building_id = building.borrow().get_id();
            if this_building_id == building_id {
                building.borrow_mut().add_to_spawn_queue(entity_type);
                return;
            }
        }
    }

    pub fn get_building_by_id(&self, id: usize) -> Option<Rc<RefCell<Building>>> {
        for building in self.buildings.iter() {
            if building.borrow().get_id() == id {
                return Some(building.clone());
            }
        }
        None
    }

    pub fn set_spawn_command_position(&mut self, building_id: usize, pos: &Vec2f) {
        for building_ref in self.buildings.iter_mut() {
            let mut building = building_ref.borrow_mut();
            if building.get_id() == building_id {
                building.set_spawn_command_position(pos.clone());
                return;
            }
        }
        println!(
            "set_spawn_command_position no building found with id {}",
            building_id
        );
    }

    pub fn get_closest_building(
        &self,
        position: &Vec2i,
        team: Option<u8>,
        not_team: Option<u8>,
        max_distance: f32,
    ) -> Option<Rc<RefCell<Building>>> {
        let mut closest_distance: f32 = 999999.0;
        let mut closest_building: Option<Rc<RefCell<Building>>> = None;

        for building_ref in self.buildings.iter() {
            let building = building_ref.borrow();
            if let Some(not_team) = not_team {
                if building.get_team() == not_team {
                    continue;
                }
            }
            if let Some(team) = team {
                if building.get_team() != team {
                    continue;
                }
            }

            let building_position = building.get_position();
            let distance = (building_position - position.clone()).as_vec2f().length();
            // TODO: This distance might not be right..
            // TODO: Perhaps should use path finding instead
            if distance < closest_distance {
                closest_distance = distance;
                closest_building = Some(building_ref.clone());
            }
        }

        if closest_distance > max_distance {
            return None;
        }

        closest_building
    }

    pub fn get_building_at(
        &self,
        position: &Vec2i,
        team: Option<u8>,
        not_team: Option<u8>,
    ) -> Option<Rc<RefCell<Building>>> {
        for building_ref in self.buildings.iter() {
            let building = building_ref.borrow();
            if building.get_position().x <= position.x
                && building.get_position().x + building.get_width() >= position.x
                && building.get_position().y <= position.y
                && building.get_position().y + building.get_height() >= position.y
            {
                if let Some(team) = team {
                    if building.get_team() != team {
                        continue;
                    }
                }
                if let Some(not_team) = not_team {
                    if building.get_team() == not_team {
                        continue;
                    }
                }
                return Some(building_ref.clone());
            }
        }
        None
    }

    pub fn remove_dead(&mut self, ground: &mut Ground) {
        self.buildings.retain(|building| {
            let building = building.borrow();
            let is_alive = building.health.is_alive();

            if !is_alive {
                for x in 0..building.get_width() {
                    for y in 0..building.get_height() {
                        ground.set_at(
                            building.get_position().x + x,
                            building.get_position().y + y,
                            GroundType::Empty,
                        )
                    }
                }
            }

            is_alive
        });
    }
}
