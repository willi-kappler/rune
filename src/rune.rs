use sdl2;
use sdl2::event::{Event, WindowEvent};
use sdl2::pixels;
use sdl2::mouse::MouseButton;

use error::{Result};

use window::{RuneWindow, RuneWindowInternal};
use canvas::{RuneCanvas};
use mouse::{RuneMouseButton};
use message::{RuneMessageBox};

pub struct Rune {
    // sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
    windows: Vec<RuneWindowInternal>,
    quit: bool,
    message_box: RuneMessageBox,
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
                message_box: RuneMessageBox::new(),
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

        rune_window.parent = self.message_box.clone();

        self.windows.push(RuneWindowInternal{
            rune_window,
            id,
            canvas: RuneCanvas{ sdl_canvas },
        });

        Ok(())
    }


    pub fn run(&mut self) -> Result<()> {
        self.quit = false;

        while !self.quit {
            for event in self.event_pump.poll_iter() {
                for window in self.windows.iter_mut() {
                    process_event(self.message_box, window, &event)?;
                }
            }

            loop {
                if let Some((sender, message)) = self.message_box.pop_message()? {
                    match message {
                        RuneMessage::ApplicationQuit {
                            self.quit = true;
                            break;
                        },
                        _ => {
                            // TODO: handle more messages
                        }
                    }
                } else {
                    // message box is empty, no more messages to process -
                    // so just leave the loop
                    break
                }
            }

            for window in self.windows.iter_mut() {
                window.process_messages()?;
            }

            // for window in self.windows.iter_mut() {
            //     window.canvas.sdl_canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
            //     window.canvas.sdl_canvas.clear();
            //     window.draw();
            //     window.canvas.sdl_canvas.present();
            // }

        }
    }
}

fn process_event(sender: &RuneMessageBox, window: &mut RuneWindowInternal, event: &sdl2::event::Event) -> Result<()> {
    match *event {
        Event::Window { timestamp: _, window_id: id, win_event: window_event } => {
            if window.id == id {
                process_window_event(sender, window, window_event)
            }
        },
        Event::MouseButtonDown { timestamp: _, window_id: id, which: _, mouse_btn: btn, x: x, y: y } => {
            if window.id == id {
                window.send_message(sender, RuneMessage::MousePress(RuneMouseButton::from(btn), x, y))
            }
        },
        Event::MouseButtonUp { timestamp: _, window_id: id, which: _, mouse_btn: btn, x: x, y: y } => {
            if window.id == id {
                window.send_message(sender, RuneMessage::MouseRelease(RuneMouseButton::from(btn), x, y))
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
                window.send_message(sender, RuneMessage::MouseMove(btn, x, y))
            }
        },
        _ => {
            // TODO: add more events...
            Ok(())
        }
    }
}

fn process_window_event(sender: &RuneMessageBox,  window: &mut RuneWindowInternal, event: sdl2::event::WindowEvent) -> Result<()> {
    window.send_message(sender, RuneMessage::From(event))
}
