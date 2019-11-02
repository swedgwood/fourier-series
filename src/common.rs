use std::{cmp, ops};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64, y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn mag(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

impl ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}