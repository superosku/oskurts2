use crate::camera::Camera;
use raqote::DrawTarget;

pub trait GameThing {
    fn update(&mut self);
}
