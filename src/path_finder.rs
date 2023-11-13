use crate::ground::Ground;
use crate::vec::{Vec2f, Vec2i};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type PathItem = (i32, i32);

pub struct Path {
    pub position_datas: HashMap<PathItem, Vec2f>,
}

impl Path {
    pub fn new(position_datas: HashMap<PathItem, Vec2f>) -> Path {
        Path { position_datas }
    }

    pub fn get_position_data(&self, position: Vec2i) -> Option<&Vec2f> {
        self.position_datas.get(&(position.x, position.y))
    }

    pub fn set_position_data(&mut self, position: Vec2i, data: Vec2f) {
        self.position_datas.insert((position.x, position.y), data);
    }
}

pub struct PathFinder {
    paths: Vec<Rc<RefCell<Path>>>,
}

impl PathFinder {
    pub fn new() -> PathFinder {
        PathFinder { paths: Vec::new() }
    }

    pub fn find_path(
        &mut self,
        ground: &Ground,
        goal: Vec2i,
        goal_width: i32,
        goal_height: i32,
        start_positions: Vec<Vec2i>,
    ) -> Option<Rc<RefCell<Path>>> {
        // let mut path_list: Vec<(PathItem, PathItem)> = Vec::new();
        let mut path_items: HashMap<PathItem, Vec2f> = HashMap::new();
        let mut unhandled_positions: Vec<PathItem> = Vec::new();

        for i in 0..goal_height {
            if !ground.blocked_at(goal.x - 1, goal.y + i) {
                path_items.insert((goal.x - 1, goal.y + i), Vec2f::new(1.0, 0.0));
                unhandled_positions.push((goal.x - 1, goal.y + i));
            }
            if !ground.blocked_at(goal.x + goal_width, goal.y + i) {
                path_items.insert((goal.x + goal_width, goal.y + i), Vec2f::new(-1.0, 0.0));
                unhandled_positions.push((goal.x + goal_width, goal.y + i));
            }
        }
        for i in 0..goal_width {
            if !ground.blocked_at(goal.x + i, goal.y - 1) {
                path_items.insert((goal.x + i, goal.y - 1), Vec2f::new(0.0, 1.0));
                unhandled_positions.push((goal.x + i, goal.y - 1));
            }
            if !ground.blocked_at(goal.x + i, goal.y + goal_height) {
                path_items.insert((goal.x + i, goal.y + goal_height), Vec2f::new(0.0, -1.0));
                unhandled_positions.push((goal.x + i, goal.y + goal_height));
            }
        }

        let mut position_index = 0;
        loop {
            if position_index >= unhandled_positions.len() {
                println!("End of search 1");
                break;
            }
            if position_index >= 1000 {
                println!("End of search 2");
                break;
            }

            let position = unhandled_positions[position_index];

            for new_position in [
                (position.0 - 1, position.1),
                (position.0 + 1, position.1),
                (position.0, position.1 - 1),
                (position.0, position.1 + 1),
            ] {
                if !ground.blocked_at(new_position.0, new_position.1)
                    && !path_items.contains_key(&new_position)
                {
                    path_items.insert(
                        new_position,
                        Vec2f::new(
                            (position.0 - new_position.0) as f32,
                            (position.1 - new_position.1) as f32,
                        ),
                    );
                    unhandled_positions.push(new_position);
                }
            }

            position_index += 1;
        }

        println!("Path list: {:?}", path_items);

        let path = Path::new(path_items);

        let path_ref = Rc::new(RefCell::new(path));
        self.paths.push(path_ref.clone());
        Some(path_ref)
    }

    pub fn find_path_simple(
        &mut self,
        ground: &Ground,
        goal: Vec2i,
        start: Vec2i,
    ) -> Option<Rc<RefCell<Path>>> {
        self.find_path(ground, goal, 1, 1, vec![start])
    }
}
