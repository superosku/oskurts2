use crate::entity::{Entity, EntityFilter};
use crate::spacial_partition::{ObjectFilter, SpacialPartition};
use crate::vec::{Vec2f, Vec2i};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct EntityContainer {
    entities_rc: Vec<Rc<RefCell<Entity>>>,
    spacial_partition: SpacialPartition<Entity, EntityFilter>,
    // entities_by_area: HashMap<(i32, i32), Vec<Rc<RefCell<Entity>>>>,
    // area_divider: u8,
}

impl EntityContainer {
    pub fn new(entities: Vec<Entity>) -> EntityContainer {
        let mut entities_rc: Vec<Rc<RefCell<Entity>>> = Vec::new();
        for entity in entities {
            entities_rc.push(Rc::new(RefCell::new(entity)));
        }
        EntityContainer {
            entities_rc,
            spacial_partition: SpacialPartition::new(8),
            // entities_by_area: HashMap::new(),
            // area_divider: 8,
        }
    }

    pub fn get_by_id(&mut self, entity_id: usize) -> &Rc<RefCell<Entity>> {
        // TODO: Fetch this from hash map and do not iterate through all of the entities
        self.entities_rc
            .iter()
            .filter(|e| e.borrow().get_id() == entity_id)
            .next()
            .unwrap()
    }

    pub fn spawn_entity(&mut self, entity: Entity) {
        self.entities_rc.push(Rc::new(RefCell::new(entity)));
    }

    pub fn iter_alive(&self) -> std::slice::Iter<Rc<RefCell<Entity>>> {
        return self.entities_rc.iter();
        // // TODO:
    }

    pub fn remove_dead(&mut self) {
        self.entities_rc.retain(|entity_rc| {
            let entity = entity_rc.borrow();
            entity.health.is_alive()
        });
    }

    pub fn entity_count(&self) -> usize {
        self.entities_rc.len()
    }

    pub fn get_entity_at_index(&mut self, index: usize) -> &Rc<RefCell<Entity>> {
        &self.entities_rc[index]
    }

    pub fn update_entities_by_area(&mut self) {
        self.spacial_partition.update_partition(&self.entities_rc);
    }

    pub fn entities_in_box(
        &self,
        top_left: Vec2f,
        bottom_right: Vec2f,
        filter: EntityFilter,
    ) -> Vec<Rc<RefCell<Entity>>> {
        self.spacial_partition.objects_in(
            ObjectFilter::InBox {
                top_left,
                bottom_right,
            },
            filter,
        )
    }

    pub fn entities_in_radius(
        &self,
        position: Vec2f,
        max_radius: f32,
        filter: EntityFilter,
    ) -> Vec<Rc<RefCell<Entity>>> {
        self.spacial_partition.objects_in(
            ObjectFilter::InRadius {
                position,
                max_radius,
            },
            filter,
        )
    }

    pub fn get_closest_entity(
        &self,
        position: Vec2f,
        max_radius: f32,
        filter: EntityFilter,
    ) -> Option<Rc<RefCell<Entity>>> {
        self.spacial_partition
            .get_closest_object(position, max_radius, filter)
    }
}
