use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use sdl2::event::WindowEvent;

use error::{Result};

pub trait RuneMessageHandler {
    pub fn process_messages(&mut self) -> Result<()> {
        Ok(())
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
struct RuneMessageBox {
    message_box: Arc<Mutex<VecDeque<(Box<RuneMessageBox>, RuneMessage)>>>,
}

impl RuneMessageBox {
    fn new() -> RuneMessageBox {
        message_box: Arc::new(Mutex::new(VecDeque::new())),
    }

    fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        let mut message_box = self.message_box.lock()?;
        message_box.push_front((Box::new(sender.clone()), message.clone()));
    }

    fn pop_message(&mut self) -> Result<Option<(RuneMessageBox, RuneMessage)>> {
        let mut message_box = self.message_box.lock()?;
        Ok(message_box.pop_back().map(|(sender, message)| (*sender, message)))
    }
}
