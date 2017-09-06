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
use event::{RuneEvent, RuneAction};

pub struct Rune {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
    windows: Vec<Rc<RefCell<RuneWindowInternal>>>,
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
            event_handler: Box::new(|e| RuneAction::None),
            on_close: Box::new(move || RuneAction::HideWindow(window_id.clone())),
        }));

        self.windows.push(internal_window.clone());

        Ok(RuneWindow{ rune_window: internal_window })
    }

    pub fn run(&mut self) {
        self.quit = false;

        let mut result_actions = Vec::new();

        while !self.quit {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Window { timestamp: _, window_id: id, win_event: we } => {
                        for w in self.windows.iter() {
                            result_actions.push(w.borrow_mut().process_window_event(id, we));
                        }
                    },
                    Event::MouseButtonDown { timestamp: _, window_id: id, which: _, mouse_btn: btn, x: x, y: y } => {
                        for w in self.windows.iter() {
                            result_actions.push(w.borrow_mut().process_mouse_down_event(id, btn, x, y));
                        }
                    },
                    Event::MouseButtonUp { timestamp: _, window_id: id, which: _, mouse_btn: btn, x: x, y: y } => {
                        for w in self.windows.iter() {
                            result_actions.push(w.borrow_mut().process_mouse_up_event(id, btn, x, y));
                        }
                    },


                    _ => {
                        // TODO: process more events...
                    }
                }
            }

            for action in result_actions.iter() {
                match *action {
                    RuneAction::ApplicationQuit => {
                        self.quit = true;
                        break;
                    },
                    RuneAction::HideWindow(id) => {
                        for w in self.windows.iter() {
                            w.borrow_mut().hide(id);
                        }
                    },
                    _ => {
                        // TODO: process more events...
                    }
                }
            }

            result_actions.clear();
        }
    }
}
