
use rune::{RuneAction, RuneMouseButton};
use canvas::RuneCanvas;
use message::{RuneMessage};

pub trait RuneWidget {
    fn draw(&mut self, canvas: &mut RuneCanvas) {
        // Do nothing in the default implementation
    }

    fn handle_message(&mut self, message: RuneMessage) {
        // Do nothing in the default implementation
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
}

impl BaseWidget {
    pub fn contains_point(&self, x: u32, y: u32) -> bool {
        (x >= self.x) && (x <= (self.x + self.width)) && (y >= self.y) && (y <= self.y + (self.height))
    }

    fn handle_message(&mut self, message: RuneMessage) {
        // TODO
    }
}
