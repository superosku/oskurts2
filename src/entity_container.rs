use crate::entity::Entity;


pub struct EntityContainer {
    entities: Vec<Entity>,
}

impl EntityContainer {
    pub fn new(entities: Vec<Entity>) -> EntityContainer {
        EntityContainer {
            entities
        }
    }

    pub fn iter_all(&self) -> std::slice::Iter<Entity> {
        self.entities.iter()
    }

    pub fn iter_all_mut(&mut self) -> std::slice::IterMut<Entity> {
        self.entities.iter_mut()
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn get_entity_at_index(&mut self, index: usize) -> Option<&mut Entity> {
        Some(&mut self.entities[index])
        // self.entities.get_mut(index)
    }
}

