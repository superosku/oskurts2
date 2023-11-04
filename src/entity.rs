use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
use rayon::spawn;
use crate::camera::Camera;
use crate::game_thing::GameThing;
use crate::vec::Vec2f;

pub struct Entity {
    position: Vec2f,
    next_position: Vec2f,
    goal: Option<Vec2f>,
    speed: f32,
    // being_stopped_for_frames: u32,
}

impl Entity {
    pub fn new(position: Vec2f) -> Entity {
        Entity {
            position: position.clone(),
            next_position: position.clone(),
            goal: Some(Vec2f::new(0.0, 0.0)),
            speed: 0.01,
            // being_stopped_for_frames: 0,
        }
    }

    pub fn get_position(&self) -> Vec2f {
        self.position.clone()
    }

    pub fn get_pushed(&mut self, other_position: Vec2f) {
        let delta = self.position.clone() - other_position;
        let delta_length = delta.length();
        if delta_length < 1.0 {
            let vector_away_from_other = delta.normalized();
            let overlap_amount = 1.0 - delta_length;
            self.next_position += vector_away_from_other * overlap_amount / 2.0;
        } else {
            // self.next_position += delta.normalized() * self.speed;
        }
    }

    pub fn flip_position(&mut self) {
        // // If we moved much less than the speed, then we're stuck, remove the goal
        // if (self.position.clone() - self.next_position.clone()).length() < self.speed * 0.5 {
        //     // self.being_stopped_for_frames += 1;
        //     // if self.being_stopped_for_frames > 3 {
        //     //     self.goal = None;
        //     // }
        // } else {
        //     // self.being_stopped_for_frames = 0;
        // }
        self.position = self.next_position.clone();
    }
}

impl GameThing for Entity {
    fn update(&mut self) {
        match &self.goal {
            Some(goal) => {
                let delta = goal.clone() - self.position.clone();
                let delta_length = delta.length();
                if delta_length < 0.1 {
                    self.goal = None;
                } else {
                    self.next_position += delta.normalized() * self.speed;
                }
            }
            None => {}
        }
    }

    fn draw(&self, dt: &mut DrawTarget, camera: &Camera) {
        let mut path_builder = PathBuilder::new();
        let draw_pos = camera.world_to_screen(&Vec2f::new(
            // self.position.x - 0.5,
            // self.position.y - 0.5,
            self.position.x,
            self.position.y,
        ));
        // path_builder.move_to(draw_pos.x, draw_pos.y);
        path_builder.arc(
            draw_pos.x,
            draw_pos.y,
            camera.length_to_pixels(0.5),
            0.0,
            2.0 * std::f32::consts::PI,
        );

        let path = path_builder.finish();
        let source = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 128, 128, 255));
        let source_dark = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 200, 200, 255));

        dt.fill(&path, &source, &DrawOptions::new());
        let stroke_style = &mut raqote::StrokeStyle::default();
        stroke_style.width = camera.length_to_pixels(0.1);
        dt.stroke(
            &path,
            &source_dark,
            stroke_style,
            &DrawOptions::new(),
        );
    }
}