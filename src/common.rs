use std::{cmp, ops};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64, pub y: f64
}

impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_ei(angle: f64) -> Self {
        Point::new(1.0, 0.0).rotate(angle)
    }

    pub fn mag(self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn angle(self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn scale(self, scale: f64) -> Self {
        Self::new(self.x * scale, self.y * scale)
    }

    pub fn rotate(self, angle: f64) -> Self {
        Self::new(
            self.x * angle.cos() - self.y * angle.sin(),
            self.x * angle.sin() + self.y * angle.cos()
        )
    }

    pub fn complex_mult(p1: Self, p2: Self) -> Self {
        Self::new(
            p1.x * p2.x - p1.y * p2.y,
            p1.x * p2.y + p1.y * p2.x
        )
    }

    pub fn x(self) -> f64 { self.x }
    
    pub fn y(self) -> f64 { self.y }
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