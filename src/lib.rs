use std::time::Duration;
use std::f64::consts::PI;

pub mod common;
pub mod canvas;
pub mod world;

use world::{SVector, World};
use canvas::{Canvas, Color, Event};

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

    pub fn draw_svectors(&mut self) {

    }

    pub fn mainloop(&mut self) {
        self.running = true;

        let mut rot = 0.0;
        while self.running {
            self.handle_events();

            self.canvas.set_draw_color((0, 0, 0));
            self.canvas.clear();
            self.canvas.set_draw_color((255, 255, 255));
            rot += 0.01;
            rot %= 2.0*PI;
            self.canvas.draw_line((0, 0), (3.0*rot.cos(), 3.0*rot.sin())).unwrap();
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        };
    }
}