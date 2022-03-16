use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Clone)]
pub struct Circle {
    pub coord: Vector,
    pub r: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self {
            coord: Vector::new(x, y),
            r,
        }
    }

    pub fn collides_with(&self, other: &Circle) -> bool {
        (self.coord - other.coord).len() < (self.r + other.r)
    }

    pub fn in_bounds(&self, l: f64, t: f64, r: f64, b: f64) -> bool {
        let x = self.coord.x;
        let y = self.coord.y;

        x > l - self.r && x < r + self.r && y > t - self.r && y < b + self.r
    }
}
