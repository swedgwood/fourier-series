extern crate sdl2;

use sdl2::event::EventPollIterator;

pub use sdl2::pixels::Color;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;
use std::ffi::NulError;
use super::common::Point;
use sdl2::rect::Point as SdlPoint;
use sdl2::mouse::MouseState;

#[derive(Debug)]
pub enum CanvasBuildError {
    IntegerOverflows(&'static str, u32),
    HeightOverflows(u32),
    WidthOverflows(u32),
    InvalidTitle(NulError),
    SdlError(String)
}

impl From<WindowBuildError> for CanvasBuildError {
    fn from(value: WindowBuildError) -> Self {
        match value {
            WindowBuildError::HeightOverflows(height) 
                => CanvasBuildError::HeightOverflows(height),
            WindowBuildError::WidthOverflows(width)
                => CanvasBuildError::WidthOverflows(width),
            WindowBuildError::InvalidTitle(err)
                => CanvasBuildError::InvalidTitle(err),
            WindowBuildError::SdlError(err)
                => CanvasBuildError::SdlError(err)
        }
    }
}

impl From<IntegerOrSdlError> for CanvasBuildError {
    fn from(value: IntegerOrSdlError) -> Self {
        match value {
            IntegerOrSdlError::IntegerOverflows(err, num)
                => CanvasBuildError::IntegerOverflows(err, num),
            IntegerOrSdlError::SdlError(err)
                => CanvasBuildError::SdlError(err)
        }
    }
}

impl From<String> for CanvasBuildError {
    fn from(value: String) -> Self {
        CanvasBuildError::SdlError(value)
    }
}

pub struct Canvas {
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
    camera_center: Point,
    scale: f64
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, CanvasBuildError> {
        let sdl_context = sdl2::init().map_err(CanvasBuildError::from)?;
        let video_subsystem = sdl_context.video().map_err(CanvasBuildError::from)?;

        let canvas = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .map_err(CanvasBuildError::from)?
            .into_canvas()
            .build()
            .map_err(CanvasBuildError::from)?;

        let event_pump = sdl_context.event_pump().map_err(CanvasBuildError::from)?;

        Ok( Self { canvas, event_pump, camera_center: Point::zero, scale: 50.0 } )
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }

    pub fn poll_events_iter(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn mouse_state(&self) -> MouseState {
        self.event_pump.mouse_state()
    }

    pub fn draw_line<T1: Into<Point>, T2: Into<Point>>(&mut self, start: T1, end: T2) -> Result<(), String> {
        self.canvas.draw_line(
            self.point_to_sdlpoint(start.into()),
            self.point_to_sdlpoint(end.into())
        )
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn set_draw_color<T: Into<Color>>(&mut self, color: T) {
        self.canvas.set_draw_color(color);
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn move_camera_by_pixels(&mut self, x: i32, y: i32) {
        let point = Point::new(-x as f64, y as f64);
        self.camera_center += point.scale(self.scale.recip());
    }

    pub fn zoom(&mut self, scale: f64, center_x: i32, center_y: i32) {
        let point_before = self.coord_to_point(center_x, center_y);
        self.scale *= scale;
        let point_after = self.coord_to_point(center_x, center_y);
        self.camera_center -= point_after - point_before;
    }

    fn point_to_sdlpoint(&self, point: Point) -> SdlPoint {
        let point = (point - self.camera_center).scale(self.scale);
        let (width, height) = self.canvas.output_size().unwrap();
        let new_x = (point.x() + (width as f64)/2.0).round() as i32;
        let new_y = (-point.y() + (height as f64)/2.0).round() as i32;
        SdlPoint::new(new_x, new_y)
    }

    fn coord_to_point(&self, x: i32, y: i32) -> Point {
        let (width, height) = self.canvas.output_size().unwrap();
        let new_x = x as f64 - (width as f64)/2.0;
        let new_y = -y as f64 + (height as f64)/2.0;
        Point::new(new_x, new_y).scale(self.scale.recip()) + self.camera_center
    }
}