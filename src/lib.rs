use std::time::Duration;
use std::f64::consts::PI;

pub mod common;
pub mod canvas;
pub mod world;

use world::{SVector, World};
use canvas::{Canvas, Color, Event};
use common::Point;

pub struct FourierSeries {
    canvas: canvas::Canvas,
    world: world::World,
    primary_color: Color,
    secondary_color: Color,
    background: Color,
    running: bool
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
            running: false
        }
    }

    pub fn handle_events(&mut self) {
        while let Some(event) = self.canvas.poll_event() {
            match event {
                Event::Quit {..} => { self.running = false; }
                Event::MouseMotion {mousestate, xrel, yrel, ..} => {
                    if mousestate.left() {
                        self.canvas.move_camera_by_pixels(xrel, yrel);
                    }
                }
                Event::MouseWheel {y, ..} => {
                    let mousestate = self.canvas.mouse_state();
                    self.canvas.zoom((1.2 as f64).powi(y), mousestate.x(), mousestate.y());
                }
                _ => {}
            }
        }
    }

    pub fn draw_svectors(&mut self, t: f64) {
        let mut last_point = Point::new(0.0, 0.0);
        self.canvas.set_draw_color(self.primary_color);
        for point in self.world.get_state(t) {
            self.canvas.draw_line(last_point, last_point+point).unwrap();
            last_point += point;
        }
    }

    pub fn mainloop(&mut self) {
        self.running = true;

        let mut time = 0.0;
        while self.running {
            self.handle_events();

            self.canvas.set_draw_color((0, 0, 0));
            self.canvas.clear();
            self.draw_svectors(time);
            self.canvas.present();

            time += 0.1 / 60.0;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        };
    }
}