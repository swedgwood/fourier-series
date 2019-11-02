use std::{cmp, ops};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64, y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
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

impl cmp::PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}