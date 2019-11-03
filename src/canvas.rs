extern crate sdl2;

use sdl2::event::EventPollIterator;

pub use sdl2::pixels::Color;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;
use std::ffi::NulError;

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
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, CanvasBuildError> {
        let sdl_context = sdl2::init().map_err(CanvasBuildError::from)?;
        let video_subsystem = sdl_context.video().map_err(CanvasBuildError::from)?;

        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .map_err(CanvasBuildError::from)?;

        let canvas = window.into_canvas().build().map_err(CanvasBuildError::from)?;
        let event_pump = sdl_context.event_pump().map_err(CanvasBuildError::from)?;

        Ok( Self { canvas, event_pump } )
    }

    pub fn poll_events_iter(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}