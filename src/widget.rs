
use rune::{RuneAction, RuneMouseButton};
use canvas::RuneCanvas;
use message::{RuneMessageBox};
use error::{Result};

pub trait RuneWidget {
    fn draw(&mut self, canvas: &mut RuneCanvas) {
        // Do nothing in the default implementation
    }

    fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        // Do nothing in the default implementation
        Ok(())
    }

    fn process_messages(&mut self) -> Result<()> {
        // Do nothing in the default implementation
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct BaseWidget {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub mouse_inside: bool,
    pub message_box: RuneMessageBox,
    pub parent: RuneMessageBox,
}

impl BaseWidget {
    pub fn contains_point(&self, x: u32, y: u32) -> bool {
        (x >= self.x) && (x <= (self.x + self.width)) && (y >= self.y) && (y <= self.y + (self.height))
    }

    fn process_messages(&mut self, message: RuneMessage) -> Result<()> {
        // TODO
        Ok(())
    }

    fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        // TODO
        Ok(())
    }

    fn set_parent(&mut self, parent: RuneMessageBox) {
        self.parent = parent.clone();
    }
}
