use std::collections::VecDeque;
use std::f64::consts::PI;
use std::iter::FromIterator;
use std::time::{Duration, Instant};

pub mod common;
pub mod canvas;
pub mod maths;
pub mod path;
pub mod svgpath;
pub mod world;

use world::{SVector, World};
use canvas::{Canvas, Color, Event, Keycode};
use common::Point;
use maths::point_average;
use path::ParametricPath;
use svgpath::SVGPath;

pub struct FourierSeries {
    canvas: canvas::Canvas,
    world: world::World,
    point_path: VecDeque<Point>,
    svector_color: Color,
    point_path_color: Color,
    background: Color,
    start_time: Instant,
    time_scale: f64,
    frame_rate: f64,
    camera_lock: bool,
    running: bool
}

impl FourierSeries {
    pub fn new() -> Self {
        let canvas = Canvas::new("Test Window", 800, 600).unwrap();
        let p = SVGPath::new("example.svg").unwrap();

        let mut svectors: Vec<SVector> = Vec::new();

        for freq in 1..100 {
            let points = (0..10000)
                .map(|x| x as f64/10000.0)
                .map(|x| Point::complex_mult(p.get_point(x), Point::from_ei(-freq as f64 * 2.0 * PI * x)));

            let point_avg = point_average(points);

            svectors.push(SVector::new(
                point_avg.angle(),
                freq as f64,
                point_avg.mag()
            ));

            let points = (0..10000)
                .map(|x| x as f64/10000.0)
                .map(|x| Point::complex_mult(p.get_point(x), Point::from_ei(freq as f64 * 2.0 * PI * x)));

            let point_avg = point_average(points);

            svectors.push(SVector::new(
                point_avg.angle(),
                -freq as f64,
                point_avg.mag()
            ));
        }
        
        let world = World::new(svectors);

        Self {
            canvas, world, point_path: VecDeque::new(),
            svector_color: Color::RGB(255, 255, 255),
            point_path_color: Color::RGB(0, 0, 255),
            background: Color::RGB(0, 0, 0),
            time_scale: 0.1,
            start_time: Instant::now(),
            frame_rate: 60.0,
            camera_lock: false,
            running: false
        }
    }

    pub fn get_background(&self) -> Color {
        self.background
    }

    pub fn get_start_time(&self) -> Instant {
        self.start_time
    }

    pub fn reset_start_time(&mut self) {
        self.start_time = Instant::now();
    } 

    pub fn get_time_scale(&self) -> f64 {
        self.time_scale
    }

    pub fn get_frame_rate(&self) -> f64 {
        self.frame_rate
    }

    pub fn set_running(&mut self, value: bool) {
        self.running = value;
    }

    pub fn get_running(&self) -> bool {
        self.running
    }
    
    pub fn present_canvas(&mut self) {
        self.canvas.present();
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.clear();
    }

    pub fn set_canvas_draw_color<T: Into<Color>>(&mut self, color: T) {
        self.canvas.set_draw_color(color);
    }

    pub fn handle_events(&mut self) {
        while let Some(event) = self.canvas.poll_event() {
            match event {
                Event::Quit {..} => { self.running = false; }
                Event::MouseMotion {mousestate, xrel, yrel, ..} => {
                    if mousestate.left() && !self.camera_lock {
                        self.canvas.move_camera_by_pixels(xrel, yrel);
                    }
                }
                Event::MouseWheel {y, ..} => {
                    let mousestate = self.canvas.mouse_state();
                    self.canvas.zoom((1.2 as f64).powi(y), mousestate.x(), mousestate.y());
                }
                Event::KeyDown { keycode, repeat, .. } => if repeat { match keycode {
                    _ => {}
                }} else { match keycode {
                    Some(Keycode::Equals) => {
                        self.time_scale *= 1.2;
                    },
                    Some(Keycode::Minus) => {
                        self.time_scale /= 1.2;
                    },
                    Some(Keycode::R) => {
                        self.point_path.clear();
                        self.start_time = Instant::now();
                    },
                    Some(Keycode::L) => {
                        self.camera_lock = !self.camera_lock;
                    },
                    _ => {}
                }},
                _ => {}
            }
        }
    }

    pub fn draw_svectors(&mut self, t: f64) -> Result<Point, String> {
        self.canvas.set_draw_color(self.svector_color);

        let mut last_point = Point::new(0.0, 0.0);
        for point in self.world.get_state(t) {
            self.canvas.draw_line(last_point, last_point+point)?;
            last_point += point;
        }
        Ok(last_point)
    }

    pub fn add_draw_point(&mut self, point: Point) {
        self.point_path.push_back(point);

        if self.point_path.len() > 1000 {
            self.point_path.pop_front();
        }
    }

    pub fn draw_point_path(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(self.point_path_color);

        let mut points = self.point_path.iter();
        if let Some(start_point) = points.next() {
            let mut last_point = *start_point;
            for point in points {
                self.canvas.draw_line(last_point, *point)?;
                last_point = *point;
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn mainloop(&mut self) {
        self.running = true;

        self.start_time = Instant::now();

        while self.running {
            let now = Instant::now();
            self.handle_events();
            let time = self.start_time.elapsed().as_secs_f64() * self.time_scale;
            println!("{}", time);

            self.canvas.set_draw_color(self.background);
            self.canvas.clear();

            let tail = self.draw_svectors(time).unwrap();
            if self.camera_lock {
                self.canvas.set_camera_pos(tail);
            }
            self.add_draw_point(tail);
            self.draw_point_path().unwrap();
            self.canvas.present();

            let framedelay = Duration::new(0, 1_000_000_000u32 / self.frame_rate as u32);
            let elapsed = now.elapsed();

            if elapsed < framedelay {
                ::std::thread::sleep(framedelay - elapsed);
            }
        };
    }
}