use crate::ground::Ground;
use crate::vec::{Vec2f, Vec2i};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type PathItem = (i32, i32);

pub struct Path {
    pub position_datas: HashMap<PathItem, Vec2f>,
}

fn get_dirs(direction: &Vec2f) -> (i32, i32) {
    let dir_y = if direction.y > 0.0 {
        1
    } else if direction.y == 0.0 {
        0
    } else {
        -1
    };
    let dir_x = if direction.x > 0.0 {
        1
    } else if direction.x == 0.0 {
        0
    } else {
        -1
    };
    (dir_x, dir_y)
}

fn two_directions_converge(
    line_1_start: &Vec2f,
    line_1_direction: &Vec2f, // Normal vector of the line from start
    line_2_start: &Vec2f,
    line_2_direction: &Vec2f, // Normal vector of the line from start
) -> bool {
    let thing1 = (line_1_start.clone() - line_2_start.clone()).length();

    let line_1_end = line_1_start.clone() + line_1_direction.clone() * 0.1;
    let line_2_end = line_2_start.clone() + line_2_direction.clone() * 0.1;

    let thing2 = (line_1_end.clone() - line_2_end.clone()).length();

    return thing1 > thing2;
}

fn get_two_lines_intersection(
    start_pos: &Vec2f,
    line_1_start: &Vec2f,
    line_1_direction: &Vec2f, // Normal vector of the line from start
    line_2_start: &Vec2f,
    line_2_direction: &Vec2f, // Normal vector of the line from start
) -> Vec2f {
    if line_1_direction.x == line_2_direction.x && line_1_direction.y == line_2_direction.y {
        return line_1_direction.clone();
    }

    let line_1_end = line_1_start + line_1_direction;
    let line_2_end = line_2_start + line_2_direction;

    let towards = if line_1_end.x - line_1_start.x == 0.0 {
        let line_2_slope = (line_2_end.y - line_2_start.y) / (line_2_end.x - line_2_start.x);
        let line_2_intercept = line_2_start.y - line_2_slope * line_2_start.x;
        let x = line_1_start.x;
        let y = line_2_slope * x + line_2_intercept;
        Vec2f::new(x, y)
    } else if (line_2_end.x - line_2_start.x) == 0.0 {
        let line_1_slope = (line_1_end.y - line_1_start.y) / (line_1_end.x - line_1_start.x);
        let line_1_intercept = line_1_start.y - line_1_slope * line_1_start.x;
        let x = line_2_start.x;
        let y = line_1_slope * x + line_1_intercept;
        Vec2f::new(x, y)
    } else {
        let line_1_slope = (line_1_end.y - line_1_start.y) / (line_1_end.x - line_1_start.x);
        let line_2_slope = (line_2_end.y - line_2_start.y) / (line_2_end.x - line_2_start.x);

        let line_1_intercept = line_1_start.y - line_1_slope * line_1_start.x;
        let line_2_intercept = line_2_start.y - line_2_slope * line_2_start.x;

        let x = (line_2_intercept - line_1_intercept) / (line_1_slope - line_2_slope);
        let y = line_1_slope * x + line_1_intercept;
        Vec2f::new(x, y)
    };

    (towards - start_pos.clone()).normalized()
}

impl Path {
    pub fn new(position_datas: HashMap<PathItem, Vec2f>) -> Path {
        Path { position_datas }
    }

    pub fn get_direction(&self, position: &Vec2i) -> Option<Vec2f> {
        let position = (position.x, position.y);
        match self.position_datas.get(&position) {
            Some(direction) => Some(direction.clone()),
            None => None,
        }
    }

    pub fn do_orienting_round(&mut self) {
        let mut new_position_datas: HashMap<PathItem, Vec2f> = HashMap::new();
        println!("Orienting round");

        for (path_item, direction) in &self.position_datas {
            let (dir_x, dir_y) = get_dirs(direction);

            // let mut neighbours: Vec<Vec2f>= Vec::new();
            let mut new_direction: Option<Vec2f> = None;

            match (dir_x, dir_y) {
                (0, 0) => {
                    println!("I do not think this should be possible :/");
                }
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => {
                    let pos_1 = (path_item.0 + dir_x, path_item.1 + 0);
                    let other_1 = self.position_datas.get(&pos_1);
                    let pos_2 = (path_item.0 + 0, path_item.1 + dir_y);
                    let other_2 = self.position_datas.get(&pos_2);
                    let pos_3 = (path_item.0 + dir_x, path_item.1 + dir_y);
                    let other_3 = self.position_datas.get(&pos_3);

                    match (other_1, other_2, other_3) {
                        (Some(other_1), Some(other_2), Some(other_3)) => {
                            if two_directions_converge(
                                &Vec2f::new(pos_1.0 as f32, pos_1.1 as f32),
                                other_1,
                                &Vec2f::new(pos_2.0 as f32, pos_2.1 as f32),
                                other_2,
                            ) {
                                new_direction = Some(get_two_lines_intersection(
                                    &Vec2f::new(path_item.0 as f32, path_item.1 as f32),
                                    &Vec2f::new(pos_1.0 as f32, pos_1.1 as f32),
                                    other_1,
                                    &Vec2f::new(pos_2.0 as f32, pos_2.1 as f32),
                                    other_2,
                                ));
                            } else if two_directions_converge(
                                &Vec2f::new(pos_3.0 as f32, pos_3.1 as f32),
                                other_3,
                                &Vec2f::new(pos_2.0 as f32, pos_2.1 as f32),
                                other_2,
                            ) {
                                new_direction = Some(get_two_lines_intersection(
                                    &Vec2f::new(path_item.0 as f32, path_item.1 as f32),
                                    &Vec2f::new(pos_3.0 as f32, pos_3.1 as f32),
                                    other_3,
                                    &Vec2f::new(pos_2.0 as f32, pos_2.1 as f32),
                                    other_2,
                                ));
                            } else if two_directions_converge(
                                &Vec2f::new(pos_1.0 as f32, pos_1.1 as f32),
                                other_1,
                                &Vec2f::new(pos_3.0 as f32, pos_3.1 as f32),
                                other_3,
                            ) {
                                new_direction = Some(get_two_lines_intersection(
                                    &Vec2f::new(path_item.0 as f32, path_item.1 as f32),
                                    &Vec2f::new(pos_1.0 as f32, pos_1.1 as f32),
                                    other_1,
                                    &Vec2f::new(pos_3.0 as f32, pos_3.1 as f32),
                                    other_3,
                                ));
                            }
                        }
                        _ => {}
                    }
                }
                (0, 1) | (1, 0) | (-1, 0) | (0, -1) => {
                    let other = self
                        .position_datas
                        .get(&(path_item.0 + dir_x, path_item.1 + dir_y));
                    if let Some(other) = other {
                        let (other_dir_x, other_dir_y) = get_dirs(other);

                        if dir_x == 0 {
                            // other_dir_x is meaningful
                            let other_2 = self
                                .position_datas
                                .get(&(path_item.0 + other_dir_x, path_item.1 + 0));
                            let other_3 = self
                                .position_datas
                                .get(&(path_item.0 + other_dir_x, path_item.1 + dir_y));
                            match (other_2, other_3) {
                                (Some(other_2), Some(other_3)) => {
                                    if two_directions_converge(
                                        &Vec2f::new(
                                            (path_item.0 + dir_x) as f32,
                                            (path_item.1 + dir_y) as f32,
                                        ),
                                        other,
                                        &Vec2f::new(
                                            (path_item.0 + other_dir_x) as f32,
                                            (path_item.1 + 0) as f32,
                                        ),
                                        other_2,
                                    ) {
                                        new_direction = Some(get_two_lines_intersection(
                                            &Vec2f::new(path_item.0 as f32, path_item.1 as f32),
                                            &Vec2f::new(
                                                (path_item.0 + dir_x) as f32,
                                                (path_item.1 + dir_y) as f32,
                                            ),
                                            other,
                                            &Vec2f::new(
                                                (path_item.0 + other_dir_x) as f32,
                                                (path_item.1 + 0) as f32,
                                            ),
                                            other_2,
                                        ));
                                    }
                                }
                                _ => {}
                            }
                        } else {
                            // other_dir_y is meaningful
                            let other_2 = self
                                .position_datas
                                .get(&(path_item.0 + 0, path_item.1 + other_dir_y));
                            let other_3 = self
                                .position_datas
                                .get(&(path_item.0 + dir_x, path_item.1 + other_dir_y));
                            match (other_2, other_3) {
                                (Some(other_2), Some(other_3)) => {
                                    if two_directions_converge(
                                        &Vec2f::new(
                                            (path_item.0 + dir_x) as f32,
                                            (path_item.1 + dir_y) as f32,
                                        ),
                                        other,
                                        &Vec2f::new(
                                            (path_item.0 + 0) as f32,
                                            (path_item.1 + other_dir_y) as f32,
                                        ),
                                        other_2,
                                    ) {
                                        new_direction = Some(get_two_lines_intersection(
                                            &Vec2f::new(path_item.0 as f32, path_item.1 as f32),
                                            &Vec2f::new(
                                                (path_item.0 + dir_x) as f32,
                                                (path_item.1 + dir_y) as f32,
                                            ),
                                            other,
                                            &Vec2f::new(
                                                (path_item.0 + 0) as f32,
                                                (path_item.1 + other_dir_y) as f32,
                                            ),
                                            other_2,
                                        ));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {
                    println!("This definitely should not be possible");
                }
            }

            if let Some(new_direction) = new_direction {
                if new_direction.x.is_nan() || new_direction.y.is_nan() {
                    println!("Why does this happen??");
                    new_position_datas.insert(*path_item, direction.clone());
                } else {
                    new_position_datas.insert(*path_item, new_direction);
                }
            } else {
                new_position_datas.insert(*path_item, direction.clone());
            }
        }

        self.position_datas = new_position_datas;
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
        start_positions: &HashSet<Vec2i>,
    ) -> Option<Rc<RefCell<Path>>> {
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
            if position_index >= 100000 {
                println!("End of search 2");
                break;
            }

            let position = unhandled_positions[position_index];

            for pos_diff in [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                let new_position = (position.0 + pos_diff.0, position.1 + pos_diff.1);

                let is_corner_cutting = pos_diff.0 != 0 && pos_diff.1 != 0;
                let corners_ok = !is_corner_cutting
                    || (!ground.blocked_at(position.0 + pos_diff.0, position.1)
                        && !ground.blocked_at(position.0, position.1 + pos_diff.1));

                if !ground.blocked_at(new_position.0, new_position.1)
                    && !path_items.contains_key(&new_position)
                    && corners_ok
                {
                    path_items.insert(
                        new_position,
                        Vec2f::new(
                            (position.0 - new_position.0) as f32,
                            (position.1 - new_position.1) as f32,
                        )
                        .normalized(), // TODO: This expensive normalize operation could be optimized out
                    );
                    unhandled_positions.push(new_position);
                }
            }

            position_index += 1;
        }

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
        let mut start_positions = HashSet::new();
        start_positions.insert(start);
        self.find_path(ground, goal, 1, 1, &start_positions)
    }
}
