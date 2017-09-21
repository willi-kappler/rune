use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use error::{Result, ResultExt, ErrorKind};
use mouse::{RuneMouseButton};

pub trait RuneMessageHandler {
    fn process_messages(&mut self, self_mb: &mut RuneMessageBox, parent_mb: &mut RuneMessageBox) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DefaultMessageHandler {
}

impl RuneMessageHandler for DefaultMessageHandler {
    fn process_messages(&mut self, self_mb: &mut RuneMessageBox, parent_mb: &mut RuneMessageBox) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RuneMessage {
    ApplicationQuit,
    WindowClose(u32),
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
    ButtonClick,
}

#[derive(Clone, Debug)]
pub struct RuneMessageBox {
    message_box: Arc<Mutex<VecDeque<(Box<RuneMessageBox>, RuneMessage)>>>,
}

impl RuneMessageBox {
    pub fn new() -> RuneMessageBox {
        RuneMessageBox {
            message_box: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        match self.message_box.lock() {
            Ok(mut message_box) => {
                message_box.push_front((Box::new(sender.clone()), message.clone()));
            }
            Err(e) => {
                bail!(ErrorKind::MutexError)
            }
        }
        Ok(())
    }

    pub fn pop_message(&mut self) -> Result<Option<(RuneMessageBox, RuneMessage)>> {
        match self.message_box.lock() {
            Ok(mut message_box) => {
                Ok(message_box.pop_back().map(|(sender, message)| (*sender, message)))
            }
            Err(e) => {
                bail!(ErrorKind::MutexError)
            }
        }
    }
}
