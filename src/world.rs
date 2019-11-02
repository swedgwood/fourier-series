use std::f64::consts::PI;

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
    pub fn new() -> Self {
        Self {
            svectors: vec![
                SVector::new(0.0, 1.0, 1.0),
                SVector::new(0.0, 2.0, 1.0)
            ]
        }
    }

    pub fn get_state(&self, t: f64) -> Vec<Point> {
        let mut cur_point = Point::new(0.0, 0.0);
        let mut points: Vec<Point> = vec![cur_point];

        for svector in &self.svectors {
            cur_point += svector.get_state(t);
            points.push(cur_point);
        }

        points
    }
}