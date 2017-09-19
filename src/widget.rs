use mouse::{RuneMouseButton};
use canvas::RuneCanvas;
use message::{RuneMessageBox, RuneMessage};
use error::{Result};

pub trait RuneWidget {
    fn draw(&mut self, canvas: &mut RuneCanvas) {
        // Do nothing in the default implementation
    }

    fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        Ok(())
    }

    fn process_messages(&mut self) -> Result<()> {
        Ok(())
    }

    fn set_parent(&mut self, parent: RuneMessageBox) {

    }
}

#[derive(Clone)]
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

    pub fn process_messages(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn set_parent(&mut self, parent: RuneMessageBox) {
        self.parent = parent;
    }
}
