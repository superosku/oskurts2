use crate::building::Building;
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
