use std::{cmp, ops};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64, pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn mag(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

impl<T1: Into<f64>, T2: Into<f64>> From<(T1, T2)> for Point {
    fn from(value: (T1, T2)) -> Self {
        Self::new(value.0.into(), value.1.into())
    }
}

impl<T: Into<Point>> ops::Add<T> for Point {
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Into<Point>> ops::AddAssign<T> for Point {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        *self = Self::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl<T: Into<Point>> ops::Sub<T> for Point {
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Into<Point>> ops::SubAssign<T> for Point {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        *self = Self::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl<T: Into<Point> + Copy> cmp::PartialEq<T> for Point {
    fn eq(&self, rhs: &T) -> bool {
        let rhs: Point = (*rhs).into();
        self.x == rhs.x && self.y == rhs.y
    }
}