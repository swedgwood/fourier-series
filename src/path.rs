use super::common::Point;

pub trait ParametricPath {
    fn get_point(&self, t: f64) -> Point;
}