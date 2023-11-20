use crate::building::Building;
use crate::entity::EntityType;
use crate::event_handler::EventHandler;
use crate::ground::{Ground, GroundType};
use crate::vec::Vec2i;

pub struct BuildingContainer {
    buildings: Vec<Building>,
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
        self.buildings.push(building);
    }

    // TODO: Remove building (set ground tiles to unoccupied)

    pub fn get_buildings(&self) -> &Vec<Building> {
        &self.buildings
    }

    pub fn update_buildings(&mut self, event_handler: &mut EventHandler) {
        for building in self.buildings.iter_mut() {
            building.update(event_handler);
        }
    }

    pub fn add_to_building_spawn_queue(&mut self, building_id: usize, entity_type: EntityType) {
        for building in self.buildings.iter_mut() {
            if building.get_id() == building_id {
                building.add_to_spawn_queue(entity_type);
                return;
            }
        }
    }

    pub fn get_building_by_id(&self, id: usize) -> Option<&Building> {
        for building in self.buildings.iter() {
            if building.get_id() == id {
                return Some(building);
            }
        }
        None
    }

    pub fn get_closest_building(&self, position: &Vec2i, team: u8) -> Option<&Building> {
        let mut closest_distance: f32 = 999999.0;
        let mut closest_building: Option<&Building> = None;

        for building in self.buildings.iter() {
            if building.get_team() != team {
                continue;
            }

            let building_position = building.get_position();
            let distance = (building_position - position.clone()).as_vec2f().length();
            // TODO: This distance might not be right..
            // TODO: Perhaps should use path finding instead
            if distance < closest_distance {
                closest_distance = distance;
                closest_building = Some(building);
            }
        }

        closest_building
    }
}
