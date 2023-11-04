use crate::vec;

#[derive(Debug, Clone)]
pub struct Camera {
    position: vec::Vec2f,
    zoom: f32,
}

pub const SCREEN_WIDTH: usize = 1000;
pub const SCREEN_HEIGHT: usize = 800;

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: vec::Vec2f::new(0.0, 0.0),
            zoom: 20.0,
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn set_position(&mut self, position: &vec::Vec2f) {
        self.position = position.clone();
    }

    pub fn world_to_screen(&self, point: &vec::Vec2f) -> vec::Vec2f {
        vec::Vec2f::new(
            point.x * self.zoom + (SCREEN_WIDTH / 2) as f32,
            point.y * self.zoom + (SCREEN_HEIGHT / 2) as f32,
        )
    }

    pub fn screen_to_world(&self, point: &vec::Vec2f) -> vec::Vec2f {
        vec::Vec2f::new(
            (point.x - (SCREEN_WIDTH / 2) as f32) / self.zoom,
            (point.y - (SCREEN_HEIGHT / 2) as f32) / self.zoom,
        )
    }

    pub fn length_to_pixels(&self, length: f32) -> f32 {
        // 100.0
        length * self.zoom
    }
}
