pub mod common;
pub mod canvas;
pub mod world;

use world::World;
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
        let canvas = Canvas::new();
        let world = World::new();

        Self {
            canvas, world,
            primary_color: Color::RGB(255, 255, 255),
            secondary_color: Color::RGB(0, 0, 255),
            background: Color::RGB(0, 0, 0),
        }
    }
}