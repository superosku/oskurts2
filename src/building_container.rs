use crate::building::Building;
use crate::ground::{Ground, GroundType};

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
}
