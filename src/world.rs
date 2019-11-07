use std::f64::consts::PI;
use std::slice::Iter;

use crate::common::Point;

pub struct SVector {
    start_angle: f64,
    frequency: f64,
    magnitude: f64,
}

impl SVector {
    pub fn new(start_angle: f64, frequency: f64, magnitude: f64) -> Self {
        Self { start_angle, frequency, magnitude }
    }

    pub fn get_state(&self, t: f64) -> Point {
        let angle = self.start_angle + 2.0 * PI * self.frequency * t;
        let x = self.magnitude * angle.cos();
        let y = self.magnitude * angle.sin();
        Point::new(x, y)
    }
}

pub struct World {
    svectors: Vec<SVector>,
}

impl World {
    pub fn new(svectors: Vec<SVector>) -> Self {
        Self { svectors }
    }

    pub fn get_state(&self, t: f64) -> WorldStateIter {
        WorldStateIter::new(self.svectors.iter(), t)
    }
}

pub struct WorldStateIter<'a> {
    svectors: Iter<'a, SVector>,
    t: f64
}

impl<'a> WorldStateIter<'a> {
    fn new(svectors: Iter<'a, SVector>, t: f64) -> Self {
        Self { svectors, t }
    }
}

impl Iterator for WorldStateIter<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(svector) = self.svectors.next() {
            Some(svector.get_state(self.t))
        } else {
            None
        }
    }
}