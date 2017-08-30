use std::rc::Rc;
use std::cell::RefCell;

use sdl2;
use sdl2::event::{Event, WindowEvent};
// use sdl2::mouse::MouseButton;
// use sdl2::keyboard::Keycode;
// use sdl2::video; // Window, WindowContext
// use sdl2::render; // Canvas

use window::{RuneWindow, RuneWindowInternal};
use widget::new_widget;
use error::{Result, ResultExt};

pub struct Rune {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
    windows: Vec<Rc<RefCell<RuneWindowInternal>>>,
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
            }
        )
    }

    pub fn new_window(&mut self, title: &str, w: u32, h: u32) -> Result<RuneWindow> {
        let sdl_window = self.video_subsystem
            .window(title, w, h)
            .position_centered()
            .resizable()
            .build()?;

        let window_id = sdl_window.id();

        let internal_window = Rc::new(RefCell::new(RuneWindowInternal {
            title: title.to_string(),
            sdl_window: sdl_window,
            widget: new_widget(window_id, 0, 0, w, h),
        }));

        self.windows.push(internal_window.clone());

        Ok(RuneWindow{ rune_window: internal_window.clone() })
    }

    pub fn run(&mut self) {
        let mut quit = false;
        let mut hidden_windows = 0;

        while !quit {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        quit = true;
                        break;
                    },
                    Event::Window { timestamp: t, window_id: id, win_event: we } => {
                        match we {
                            WindowEvent::Close => {
                                for w in self.windows.iter() {
                                    let w_id = w.borrow().widget.id;
                                    if w_id == id {
                                        w.borrow_mut().sdl_window.hide();
                                        hidden_windows = hidden_windows + 1;
                                        if hidden_windows == self.windows.len() {
                                            quit = true;
                                        }
                                        break;
                                    }
                                }
                            },
                            _ => {
                                // TODO: process more events...
                            }
                        }
                    },
                    _ => {
                        // TODO: process more events...
                    }
                }
            }
        }
    }
}
