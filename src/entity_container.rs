use crate::entity::Entity;
use crate::vec::Vec2f;
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Map;
use std::path::Iter;
use std::rc::Rc;

pub struct EntityContainer {
    entities_rc: Vec<Rc<RefCell<Entity>>>,
    entities_by_area: HashMap<(i32, i32), Vec<Rc<RefCell<Entity>>>>,
    area_divider: u8,
}

impl EntityContainer {
    pub fn new(entities: Vec<Entity>) -> EntityContainer {
        let mut entities_rc: Vec<Rc<RefCell<Entity>>> = Vec::new();
        for entity in entities {
            entities_rc.push(Rc::new(RefCell::new(entity)));
        }
        EntityContainer {
            entities_rc,
            entities_by_area: HashMap::new(),
            area_divider: 8,
        }
    }

    pub fn iter_all(&self) -> std::slice::Iter<Rc<RefCell<Entity>>> {
        return self.entities_rc.iter();
    }

    pub fn iter_all_mut(&mut self) -> std::slice::IterMut<Rc<RefCell<Entity>>> {
        return self.entities_rc.iter_mut();
    }

    pub fn entity_count(&self) -> usize {
        self.entities_rc.len()
    }

    pub fn get_entity_at_index(&mut self, index: usize) -> &Rc<RefCell<Entity>> {
        &self.entities_rc[index]
    }

    fn position_to_area(&self, position: &Vec2f) -> (i32, i32) {
        (
            position.x as i32 / self.area_divider as i32,
            position.y as i32 / self.area_divider as i32,
        )
    }

    pub fn update_entities_by_area(&mut self) {
        self.entities_by_area.clear();
        for entity_rc in self.entities_rc.iter() {
            let entity = entity_rc.borrow();
            let entity_position = entity.get_position();

            let entity_area = self.position_to_area(&entity_position);

            match self.entities_by_area.get_mut(&entity_area) {
                Some(area_slot) => {
                    area_slot.push(entity_rc.clone());
                }
                None => {
                    let mut new_vec = Vec::new();
                    new_vec.push(entity_rc.clone());
                    self.entities_by_area.insert(entity_area, new_vec);
                }
            }
        }
        // for (area, entities) in self.entities_by_area.iter() {
        //     println!("Area {:?} has {} entities", area, entities.len());
        // }
    }

    pub fn entities_in_radius(
        &self,
        position: &Vec2f,
        max_radius: f32,
    ) -> Vec<Rc<RefCell<Entity>>> {
        let min_x = (position.x - max_radius) as i32 / self.area_divider as i32;
        let max_x = (position.x + max_radius) as i32 / self.area_divider as i32;
        let min_y = (position.y - max_radius) as i32 / self.area_divider as i32;
        let max_y = (position.y + max_radius) as i32 / self.area_divider as i32;

        let mut entities_in_radius: Vec<Rc<RefCell<Entity>>> = Vec::new();

        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                match self.entities_by_area.get(&(x, y)) {
                    Some(entities) => {
                        for entity_rc in entities.iter() {
                            let entity = entity_rc.borrow();
                            let entity_position = entity.get_position();
                            let distance = (entity_position - position.clone()).length();
                            if distance < max_radius {
                                entities_in_radius.push(entity_rc.clone());
                            }
                        }
                    }
                    None => {}
                }
            }
        }

        entities_in_radius
    }

    pub fn get_closest_entity(
        &self,
        position: &Vec2f,
        max_radius: f32,
    ) -> Option<&Rc<RefCell<Entity>>> {
        let min_x = (position.x - max_radius) as i32 / self.area_divider as i32;
        let max_x = (position.x + max_radius) as i32 / self.area_divider as i32;
        let min_y = (position.y - max_radius) as i32 / self.area_divider as i32;
        let max_y = (position.y + max_radius) as i32 / self.area_divider as i32;

        let mut closest_distance: f32 = 999999.0;
        let mut closest_entity: Option<&Rc<RefCell<Entity>>> = None;

        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                match self.entities_by_area.get(&(x, y)) {
                    Some(entities) => {
                        for entity_rc in entities.iter() {
                            let entity = entity_rc.borrow();
                            let entity_position = entity.get_position();
                            let distance = (entity_position - position.clone()).length();
                            if distance < closest_distance && distance < max_radius {
                                closest_distance = distance;
                                closest_entity = Some(entity_rc);
                            }
                        }
                    }
                    None => {}
                }
            }
        }

        closest_entity
    }

    pub fn get_closest_entity_brute_force(
        &self,
        position: &Vec2f,
        max_radius: f32,
    ) -> Option<&Rc<RefCell<Entity>>> {
        let mut closest_distance: f32 = 999999.0;
        let mut closest_entity: Option<&Rc<RefCell<Entity>>> = None;
        for entity_rc in self.entities_rc.iter() {
            let entity = entity_rc.borrow();
            let entity_position = entity.get_position();
            let distance = (entity_position - position.clone()).length();
            if distance < closest_distance && distance < max_radius {
                closest_distance = distance;
                closest_entity = Some(entity_rc);
            }
        }

        closest_entity
    }
}
