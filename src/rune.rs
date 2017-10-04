use sdl2;
use sdl2::event::{Event, WindowEvent};
use sdl2::pixels;
use sdl2::mouse::MouseButton;

use error::{Result};

use window::{RuneWindow, RuneWindowInternal};
use canvas::{RuneCanvas};

#[derive(Clone, Copy)]
pub enum RuneAction {
    ApplicationQuit,
    WindowHide,
}

#[derive(Clone, Copy)]
pub enum RuneMouseButton {
    Left,
    Middle,
    Right,
    Unknown
}

impl From<MouseButton> for RuneMouseButton {
    fn from(btn: MouseButton) -> RuneMouseButton {
        match btn {
            MouseButton::Left => RuneMouseButton::Left,
            MouseButton::Middle => RuneMouseButton::Middle,
            MouseButton::Right => RuneMouseButton::Right,
            _ => RuneMouseButton::Unknown
        }
    }
}

pub struct Rune {
    // sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
    sdl_ttf: sdl2::ttf::Sdl2TtfContext,
    windows: Vec<RuneWindowInternal>,
    quit: bool,
}

impl Rune {
    pub fn init() -> Result<Rune> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let event_pump = sdl_context.event_pump()?;
        let sdl_ttf = sdl2::ttf::init()?;

        Ok(
            Rune {
                // sdl_context: sdl_context,
                video_subsystem: video_subsystem,
                event_pump: event_pump,
                sdl_ttf,
                windows: Vec::new(),
                quit: false,
            }
        )
    }

    pub fn add_window(&mut self, rune_window: RuneWindow) -> Result<()> {
        let sdl_window = self.video_subsystem
            .window(&rune_window.title, rune_window.width, rune_window.height)
            .resizable()
            .build()?;

        let id = sdl_window.id();

        let mut sdl_canvas = sdl_window.into_canvas().build()?;
        sdl_canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        sdl_canvas.clear();
        sdl_canvas.present();

        self.windows.push(RuneWindowInternal{
            rune_window,
            id,
            canvas: RuneCanvas{ sdl_canvas },
        });

        Ok(())
    }


    pub fn run(&mut self) {
        self.quit = false;

        for window in self.windows.iter_mut() {
            window.canvas.sdl_canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
            window.canvas.sdl_canvas.clear();
            window.draw();
            window.canvas.sdl_canvas.present();
        }

        while !self.quit {
            for event in self.event_pump.poll_iter() {
                for window in self.windows.iter_mut() {
                    match process_event(window, &event) {
                        Some(RuneAction::ApplicationQuit) => {
                            self.quit = true
                        },
                        Some(RuneAction::WindowHide) => {
                            window.canvas.sdl_canvas.window_mut().hide();
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

fn process_event(window: &mut RuneWindowInternal, event: &sdl2::event::Event) -> Option<RuneAction> {
    match *event {
        Event::Window { timestamp: _, window_id: id, win_event: window_event } => {
            if window.id == id {
                process_window_event(window, window_event)
            } else {
                None
            }
        },
        Event::MouseButtonDown { timestamp: _, window_id: id, which: _, mouse_btn: btn, x, y } => {
            if window.id == id {
                process_mouse_press_event(window, btn.into(), x, y)
            } else {
                None
            }
        },
        Event::MouseButtonUp { timestamp: _, window_id: id, which: _, mouse_btn: btn, x, y } => {
            if window.id == id {
                process_mouse_release_event(window, btn.into(), x, y)
            } else {
                None
            }
        }
        Event::MouseMotion { timestamp: _, window_id: id, which: _, mousestate: state, x, y, xrel: dx, yrel: dy } => {
            if window.id == id {
                let btn = if state.left() {
                    RuneMouseButton::Left
                } else if state.middle() {
                    RuneMouseButton::Middle
                } else if state.right() {
                    RuneMouseButton::Right
                } else {
                    RuneMouseButton::Unknown
                };
                process_mouse_move_event(window, btn, x, y)
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

fn process_window_event(window: &mut RuneWindowInternal, event: sdl2::event::WindowEvent)  -> Option<RuneAction> {
    match event {
        WindowEvent::Close => {
            (window.rune_window.event_handler).on_close()
        },
        WindowEvent::Moved(x, y) => {
            (window.rune_window.event_handler).on_move(x, y)
        },
        WindowEvent::Resized(w, h) => {
            window.canvas.sdl_canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
            window.canvas.sdl_canvas.clear();
            window.draw();
            let result =  (window.rune_window.event_handler).on_resize(w, h);
            window.canvas.sdl_canvas.present();
            result
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


fn process_mouse_press_event(window: &mut RuneWindowInternal, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
    // println!("rune.rs: process_mouse_press_event: {}, {}", x, y);
    window.mouse_press(mouse_button, x as u32, y as u32)
}

fn process_mouse_release_event(window: &mut RuneWindowInternal, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
    // println!("rune.rs: process_mouse_release_event: {}, {}", x, y);
    window.mouse_release(mouse_button, x as u32, y as u32)
}

fn process_mouse_move_event(window: &mut RuneWindowInternal, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
    window.mouse_move(mouse_button, x as u32, y as u32)
}
