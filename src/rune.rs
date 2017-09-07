use sdl2;
use sdl2::event::{Event, WindowEvent};
use error::{Result};

use window::{RuneWindow, RuneWindowInternal};

pub enum RuneAction {
    ApplicationQuit,
    WindowHide,
}

pub enum RuneMouseButton {
    Left,
    Middle,
    Right,
    Unknown
}

pub struct Rune {
    // sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
    windows: Vec<RuneWindowInternal>,
    quit: bool,
}

impl Rune {
    pub fn init() -> Result<Rune> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let event_pump = sdl_context.event_pump()?;

        Ok(
            Rune {
                // sdl_context: sdl_context,
                video_subsystem: video_subsystem,
                event_pump: event_pump,
                windows: Vec::new(),
                quit: false,
            }
        )
    }

    pub fn add_window(&mut self, w: RuneWindow) -> Result<()> {
        let sdl_window = self.video_subsystem
            .window(&w.title, w.w, w.h)
            .resizable()
            .build()?;

        let window_id = sdl_window.id();

        self.windows.push(RuneWindowInternal{
            rune_window: w,
            id: window_id,
            sdl_window,
        });

        Ok(())
    }


    pub fn run(&mut self) {
        self.quit = false;

        while !self.quit {
            for event in self.event_pump.poll_iter() {
                for window in self.windows.iter_mut() {
                    match process_events(window, &event) {
                        Some(RuneAction::ApplicationQuit) => {
                            self.quit = true
                        },
                        _ => {
                            // Nothing else for now...
                        }
                    }
                }
            }
        }
    }
}

fn process_events(window: &mut RuneWindowInternal, event: &sdl2::event::Event) -> Option<RuneAction> {
    match *event {
        Event::Window { timestamp: _, window_id: id, win_event: window_event } => {
            if window.id == id {
                process_window_events(window, window_event)
            } else {
                None
            }
        },
        _ => {
            // TODO: add more events...
            None
        }
    }
}

fn process_window_events(window: &mut RuneWindowInternal, event: sdl2::event::WindowEvent)  -> Option<RuneAction> {
    match event {
        WindowEvent::Close => {
            let action = (window.rune_window.event_handler).on_close();

            match action {
                Some(RuneAction::WindowHide) => {
                    window.sdl_window.hide();
                    None
                },
                _ => {
                    action // Return unhandled action to caller
                }
            }
        },
        WindowEvent::Moved(x, y) => {
            (window.rune_window.event_handler).on_move(x, y)
        },
        WindowEvent::Resized(w, h) => {
            (window.rune_window.event_handler).on_resize(w, h)
        },
        WindowEvent::Minimized => {
            (window.rune_window.event_handler).on_minimize()
        },
        WindowEvent::Maximized => {
            (window.rune_window.event_handler).on_maximize()
        },
        WindowEvent::Enter => {
            (window.rune_window.event_handler).on_enter()
        },
        WindowEvent::Leave => {
            (window.rune_window.event_handler).on_leave()
        },
        _ => {
            // TODO: add more events...
            None
        }
    }
}
