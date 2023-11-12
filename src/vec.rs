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

    pub fn as_vec2i(&self) -> Vec2i {
        Vec2i::new(self.x as i32, self.y as i32)
    }
}

impl ops::Add for Vec2f {
    type Output = Vec2f;

    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a, 'b> ops::Add<&'b Vec2f> for &'a Vec2f {
    type Output = Vec2f;

    fn add(self, other: &'b Vec2f) -> Vec2f {
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

#[derive(Debug, Clone)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x: x, y: y }
    }

    // pub fn length(&self) -> i32 {
    //     (self.x.powi(2) + self.y.powi(2)).sqrt()
    // }

    // pub fn normalized(&self) -> Vec2i {
    //     let length = self.length();
    //     if length == 0 {
    //         return Vec2i::new(0, 0); // Todo is this best way?
    //     }
    //     Vec2i::new(self.x / length, self.y / length)
    // }

    pub fn as_vec2f(&self) -> Vec2f {
        Vec2f::new(self.x as f32, self.y as f32)
    }
}

impl ops::Add for Vec2i {
    type Output = Vec2i;

    fn add(self, other: Vec2i) -> Vec2i {
        Vec2i::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a, 'b> ops::Add<&'b Vec2i> for &'a Vec2i {
    type Output = Vec2i;

    fn add(self, other: &'b Vec2i) -> Vec2i {
        Vec2i::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Neg for Vec2i {
    type Output = Vec2i;

    fn neg(self) -> Vec2i {
        Vec2i::new(-self.x, -self.y)
    }
}

impl ops::Sub for Vec2i {
    type Output = Vec2i;

    fn sub(self, other: Vec2i) -> Vec2i {
        Vec2i::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<i32> for Vec2i {
    type Output = Vec2i;

    fn mul(self, times: i32) -> Vec2i {
        Vec2i::new(self.x * times, self.y * times)
    }
}

impl ops::AddAssign for Vec2i {
    fn add_assign(&mut self, other: Vec2i) {
        *self = Vec2i::new(self.x + other.x, self.y + other.y);
    }
}

impl ops::Div<i32> for Vec2i {
    type Output = Vec2i;

    fn div(self, times: i32) -> Vec2i {
        Vec2i::new(self.x / times, self.y / times)
    }
}
