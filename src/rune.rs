use sdl2;
use sdl2::event::Event;
// use sdl2::mouse::MouseButton;
// use sdl2::keyboard::Keycode;
// use sdl2::video; // Window, WindowContext
// use sdl2::render; // Canvas

use window::Window;
use widget::new_widget;
use error::{Result, ResultExt};

pub struct Rune {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
}

impl Rune {
    pub fn init() -> Result<Rune> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let event_pump = sdl_context.event_pump()?;

        Ok(
            Rune {
                sdl_context: sdl_context,
                video_subsystem: video_subsystem,
                event_pump: event_pump,
            }
        )
    }

    pub fn new_window(&self, title: &str, w: u32, h: u32) -> Result<Window> {
        let sdl_window = self.video_subsystem
            .window(title, w, h)
            .position_centered()
            .resizable()
            .build()?;

        Ok(Window {
            title: title.to_string(),
            sdl_window: sdl_window,
            widget: new_widget(0, 0, w, h),
        })
    }

    pub fn run(&mut self) {
        let mut quit = false;

        while !quit {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        quit = true;
                        break;
                    }
                    _ => {
                        // TODO: process more events...
                    }
                }
            }
        }
    }
}
