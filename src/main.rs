use std::time::Duration;

mod canvas;
use canvas::{Canvas, Color, Event, Keycode};

const primary_color: Color = Color {r:255, g:255, b:255, a:255};
const secondary_color: Color = Color {r:0, g:0, b:255, a:255};
const background: Color = Color {r:0, g:0, b:0, a:255};

fn main() {
    let mut canvas = Canvas::new();

    let mut running = true;
    while running {
        for event in canvas.poll_events_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    };
}
