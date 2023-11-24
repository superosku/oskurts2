use crate::entity::EntityFilter;
use crate::vec::Vec2f;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait SpaciallyPartitionable<R> {
    fn get_position(&self) -> Vec2f;
    fn matches_filter(&self, filter: &R) -> bool;
}

pub enum ObjectFilter {
    InBox {
        top_left: Vec2f,
        bottom_right: Vec2f,
    },
    InRadius {
        position: Vec2f,
        max_radius: f32,
    },
}

pub struct SpacialPartition<T, R>
where
    T: SpaciallyPartitionable<R>,
{
    objects_by_area: HashMap<(i32, i32), Vec<Rc<RefCell<T>>>>,
    area_divider: u8,
    phantom: std::marker::PhantomData<R>,
}

impl<T: SpaciallyPartitionable<R>, R> SpacialPartition<T, R>
where
    T: SpaciallyPartitionable<R>,
{
    pub fn new(area_divider: u8) -> SpacialPartition<T, R> {
        SpacialPartition {
            objects_by_area: HashMap::new(),
            area_divider,
            phantom: std::marker::PhantomData,
        }
    }

    fn position_to_area(&self, position: &Vec2f) -> (i32, i32) {
        (
            position.x as i32 / self.area_divider as i32,
            position.y as i32 / self.area_divider as i32,
        )
    }

    pub fn update_partition(&mut self, objects: &Vec<Rc<RefCell<T>>>) {
        self.objects_by_area.clear();
        for object_rc in objects.iter() {
            let object = object_rc.borrow();
            let object_position = object.get_position();

            let entity_area = self.position_to_area(&object_position);

            match self.objects_by_area.get_mut(&entity_area) {
                Some(area_slot) => {
                    area_slot.push(object_rc.clone());
                }
                None => {
                    let mut new_vec = Vec::new();
                    new_vec.push(object_rc.clone());
                    self.objects_by_area.insert(entity_area, new_vec);
                }
            }
        }
    }

    pub fn objects_in(&self, objects_in: ObjectFilter, filter: R) -> Vec<Rc<RefCell<T>>> {
        let mut objects_in_radius: Vec<Rc<RefCell<T>>> = Vec::new();

        let (min_x, max_x, min_y, max_y) = match &objects_in {
            ObjectFilter::InBox {
                top_left,
                bottom_right,
            } => {
                let min_x = top_left.x as i32 / self.area_divider as i32;
                let max_x = bottom_right.x as i32 / self.area_divider as i32;
                let min_y = top_left.y as i32 / self.area_divider as i32;
                let max_y = bottom_right.y as i32 / self.area_divider as i32;
                (min_x, max_x, min_y, max_y)
            }
            ObjectFilter::InRadius {
                position,
                max_radius,
            } => {
                let min_x = (position.x - max_radius).floor() as i32 / self.area_divider as i32;
                let max_x = (position.x + max_radius).ceil() as i32 / self.area_divider as i32;
                let min_y = (position.y - max_radius).floor() as i32 / self.area_divider as i32;
                let max_y = (position.y + max_radius).ceil() as i32 / self.area_divider as i32;
                (min_x, max_x, min_y, max_y)
            }
        };

        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                match self.objects_by_area.get(&(x, y)) {
                    Some(entities) => {
                        for object_rc in entities.iter() {
                            let object = object_rc.borrow();
                            let object_position = object.get_position();

                            if !object.matches_filter(&filter) {
                                continue;
                            }

                            match &objects_in {
                                ObjectFilter::InBox {
                                    top_left,
                                    bottom_right,
                                } => {
                                    if object_position.x >= top_left.x
                                        && object_position.x <= bottom_right.x
                                        && object_position.y >= top_left.y
                                        && object_position.y <= bottom_right.y
                                    {
                                        objects_in_radius.push(object_rc.clone());
                                    }
                                }
                                ObjectFilter::InRadius {
                                    position,
                                    max_radius,
                                } => {
                                    let distance = (object_position - position.clone()).length();
                                    if distance < *max_radius {
                                        objects_in_radius.push(object_rc.clone());
                                    }
                                }
                            };
                        }
                    }
                    None => {}
                }
            }
        }

        objects_in_radius
    }

    pub fn get_closest_object(
        &self,
        position: Vec2f,
        max_radius: f32,
        filter: R,
    ) -> Option<Rc<RefCell<T>>> {
        let mut closest_distance: f32 = f32::MAX;
        let mut closest_entity: Option<Rc<RefCell<T>>> = None;

        for object_rc in self.objects_in(
            ObjectFilter::InRadius {
                position: position.clone(),
                max_radius,
            },
            filter,
        ) {
            let object = object_rc.borrow();

            let object_position = object.get_position();
            let distance = (object_position - position.clone()).length();
            if distance < closest_distance && distance < max_radius {
                closest_distance = distance;
                closest_entity = Some(object_rc.clone());
            }
        }

        closest_entity
    }
}
