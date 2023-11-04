use raqote::DrawTarget;
use crate::camera::Camera;

pub trait GameThing {
    fn update(&mut self);
    fn draw(&self, dt: &mut DrawTarget, camera: &Camera);
}
