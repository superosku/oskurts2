use crate::constants::{GROUND_HEIGHT, GROUND_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_HW_RATIO};
use crate::vec;

#[derive(Debug, Clone)]
pub struct Camera {
    position: vec::Vec2f,
    zoom: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: vec::Vec2f::new(GROUND_WIDTH as f32 / 2.0, GROUND_HEIGHT as f32 / 2.0),
            zoom: 20.0,
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn set_position(&mut self, position: &vec::Vec2f) {
        self.position = position.clone();
    }

    pub fn move_position(&mut self, delta: &vec::Vec2f) {
        self.position += delta.clone() / self.zoom * 10.0;
    }

    pub fn zoom(&mut self, amount: f32) {
        self.zoom *= amount;
    }

    pub fn world_to_screen(&self, point: &vec::Vec2f) -> vec::Vec2f {
        vec::Vec2f::new(
            point.x * self.zoom + (SCREEN_WIDTH / 2) as f32
                - self.length_to_pixels_x(self.position.x),
            point.y * self.zoom * TILE_HW_RATIO + (SCREEN_HEIGHT / 2) as f32
                - self.length_to_pixels_y(self.position.y),
        )
    }

    pub fn screen_to_world(&self, point: &vec::Vec2f) -> vec::Vec2f {
        vec::Vec2f::new(
            ((point.x + self.length_to_pixels_x(self.position.x)) - (SCREEN_WIDTH / 2) as f32)
                / self.zoom,
            ((point.y + self.length_to_pixels_y(self.position.y)) - (SCREEN_HEIGHT / 2) as f32)
                / self.zoom
                / TILE_HW_RATIO,
        )
    }

    pub fn length_to_pixels_x(&self, length: f32) -> f32 {
        length * self.zoom
    }

    pub fn length_to_pixels_y(&self, length: f32) -> f32 {
        length * self.zoom * TILE_HW_RATIO
    }

    pub fn length_to_pixels(&self, length: f32) -> f32 {
        length * self.zoom
    }

    // pub fn pixels_to_length(&self, pixels: f32) -> f32 {
    //     pixels / self.zoom
    // }
}
