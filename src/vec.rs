use std::ops;

#[derive(Debug, Clone)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x: x, y: y }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Vec2f {
        let length = self.length();
        if length == 0.0 {
            return Vec2f::new(0.0, 0.0); // Todo is this best way?
        }
        Vec2f::new(self.x / length, self.y / length)
    }
    //
    // pub fn added(&self, other: &Vec2f) -> Vec2f {
    //     Vec2f::new(self.x + other.x, self.y + other.y)
    // }
    //
    // pub fn subtracted(&self, other: &Vec2f) -> Vec2f {
    //     Vec2f::new(self.x - other.x, self.y - other.y)
    // }
    //
    // pub fn multiplied(&self, times: f32) -> Vec2f {
    //     Vec2f::new(self.x * times, self.y * times)
    // }
    //
    // pub fn negated(&self) -> Vec2f {
    //     Vec2f::new(-self.x, -self.y)
    // }
}

impl ops::Add for Vec2f {
    type Output = Vec2f;

    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Vec2f {
        Vec2f::new(-self.x, -self.y)
    }
}

impl ops::Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, times: f32) -> Vec2f {
        Vec2f::new(self.x * times, self.y * times)
    }
}

impl ops::AddAssign for Vec2f {
    fn add_assign(&mut self, other: Vec2f) {
        *self = Vec2f::new(self.x + other.x, self.y + other.y);
    }
}

impl ops::Div<f32> for Vec2f {
    type Output = Vec2f;

    fn div(self, times: f32) -> Vec2f {
        Vec2f::new(self.x / times, self.y / times)
    }
}
