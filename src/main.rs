mod canvas;
use canvas::{Canvas, Event, Keycode};

fn main() {
    let mut canvas = Canvas::new();

    let mut running = true;
    while running {
        for event in canvas.poll_events_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                }
                _ => {}
            }
        }
    };
}
