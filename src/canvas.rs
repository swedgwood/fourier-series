extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::EventPollIterator;
use std::time::Duration;

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;

pub struct Canvas {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
}

impl Canvas {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Test Window", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self { sdl_context, video_subsystem, canvas, event_pump }
    }


    pub fn poll_events_iter(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}