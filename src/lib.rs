pub mod common;
pub mod canvas;
pub mod world;

#[cfg(test)]
mod tests;

use world::{SVector, World};
use canvas::{Canvas, Color};

pub struct FourierSeries {
    canvas: canvas::Canvas,
    world: world::World,
    primary_color: Color,
    secondary_color: Color,
    background: Color,
}

impl FourierSeries {
    pub fn new() -> Self {
        let canvas = Canvas::new("Test Window", 800, 600).unwrap();
        let world = World::new(vec![
                SVector::new(0.0, 1.0, 1.0),
                SVector::new(0.0, 2.0, 1.0)
        ]);

        Self {
            canvas, world,
            primary_color: Color::RGB(255, 255, 255),
            secondary_color: Color::RGB(0, 0, 255),
            background: Color::RGB(0, 0, 0),
        }
    }
}