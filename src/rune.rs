use sdl2;
use sdl2::event::{Event, WindowEvent};
use error::{Result, ResultExt};

use window::{RuneWindow, RuneWindowInternal, RuneWindowAction};

struct RuneAction {
    window_id: u32,
    window_action: RuneWindowAction,
}

pub struct Rune {
    sdl_context: sdl2::Sdl,
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
                sdl_context: sdl_context,
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

        let mut rune_actions = Vec::new();

        while !self.quit {
            for event in self.event_pump.poll_iter() {
                for window in self.windows.iter_mut() {
                    let action = RuneAction {
                        window_id: window.id,
                        window_action: process_events(window, &event),
                    };
                    rune_actions.push(action);
                }
            }

            for action in rune_actions.iter() {
                for window in self.windows.iter_mut() {
                    match action.window_action {
                        RuneWindowAction::Hide => {
                            if action.window_id == window.id {
                                window.sdl_window.hide()
                            }
                        },
                        RuneWindowAction::Close => {
                            self.quit = true;
                            break;
                        },
                        _ => {
                            // TODO: add more events...
                        }
                    }
                }
            }

            rune_actions.clear();
        }
    }
}

fn process_events(window: &mut RuneWindowInternal, event: &sdl2::event::Event) -> RuneWindowAction {
    match *event {
        Event::Window { timestamp: _, window_id: id, win_event: window_event } => {
            if window.id == id {
                process_window_events(window, window_event)
            } else {
                RuneWindowAction::None
            }
        },
        _ => {
            // TODO: add more events...
            RuneWindowAction::None
        }
    }
}

fn process_window_events(window: &mut RuneWindowInternal, event: sdl2::event::WindowEvent)  -> RuneWindowAction {
    match event {
        WindowEvent::Close => {
            (window.rune_window.event_handler).on_close()
        },
        _ => {
            // TODO: add more events...
            RuneWindowAction::None
        }
    }
}
