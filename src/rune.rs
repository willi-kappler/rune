use sdl2;
use sdl2::event::{Event, WindowEvent};
use sdl2::pixels;
use sdl2::mouse::MouseButton;

use error::{Result};

use window::{RuneWindow, RuneWindowInternal};
use canvas::{RuneCanvas};

pub trait RuneMessageHandler {
    pub fn handle_message(&mut self, message: RuneMessage) {
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DefaultMessageHandler {
}

#[derive(Clone, Copy, Debug)]
pub enum RuneMessage {
    ApplicationQuit,
    WindowClose,
    WindowMove(u32, u32),
    WindowResize(u32, u32),
    WindowMinimize,
    WindowMaximize,
    WindowEnter,
    WindowLeave,
    WindowUnknown,
    MousePress(RuneMouseButton, u32, u32),
    MouseRelease(RuneMouseButton, u32, u32),
    MouseMove(RuneMouseButton, u32, u32),
}

impl From<sdl2::event::WindowEvent> for RuneMessage {
    fn from(win_evt: sdl2::event::WindowEvent) -> RuneMessage {
        match event {
            WindowEvent::Close => {
                RuneMessage::WindowClose
            },
            WindowEvent::Moved(x, y) => {
                RuneMessage::WindowMove()
            },
            WindowEvent::Resized(w, h) => {
                RuneMessage::WindowResize()
            },
            WindowEvent::Minimized => {
                RuneMessage::WindowMinmize
            },
            WindowEvent::Maximized => {
                RuneMessage::WindowMaximize
            },
            WindowEvent::Enter => {
                RuneMessage::WindowEnter
            },
            WindowEvent::Leave => {
                RuneMessage::WindowLeave
            },
            _ => {
                // TODO: add more events...
                RuneMessage::WindowUnknown
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

        while !self.quit {
            for event in self.event_pump.poll_iter() {
                for window in self.windows.iter_mut() {
                    process_event(window, &event)
                }
            }

            for window in self.windows.iter_mut() {
                window.canvas.sdl_canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
                window.canvas.sdl_canvas.clear();
                window.draw();
                window.canvas.sdl_canvas.present();

            }
        }
    }
}

fn process_event(window: &mut RuneWindowInternal, event: &sdl2::event::Event) {
    match *event {
        Event::Window { timestamp: _, window_id: id, win_event: window_event } => {
            if window.id == id {
                process_window_event(window, window_event)
            }
        },
        Event::MouseButtonDown { timestamp: _, window_id: id, which: _, mouse_btn: btn, x: x, y: y } => {
            if window.id == id {
                window.handle_message(RuneMessage::MousePress(RuneMouseButton::from(btn), x, y));
            }
        },
        Event::MouseButtonUp { timestamp: _, window_id: id, which: _, mouse_btn: btn, x: x, y: y } => {
            if window.id == id {
                window.handle_message(RuneMessage::MouseRelease(RuneMouseButton::from(btn), x, y));
            }
        }
        Event::MouseMotion { timestamp: _, window_id: id, which: _, mousestate: state, x: x, y: y, xrel: dx, yrel: dy } => {
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
                window.handle_message(RuneMessage::MouseMove(btn, x, y));
            }
        },
        _ => {
            // TODO: add more events...
        }
    }
}

fn process_window_event(window: &mut RuneWindowInternal, event: sdl2::event::WindowEvent) {
    window.handle_message(RuneMessage::From(event))
}
