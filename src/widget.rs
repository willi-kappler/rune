
use rune::{RuneAction, RuneMouseButton};
use canvas::RuneCanvas;

pub trait RuneWidget {
    fn draw(&mut self, canvas: &mut RuneCanvas) {
        // Do nothing in the default implementation
    }

    fn contains_point(&mut self, x: u32, y: u32) -> bool;

    fn x(&mut self) -> u32;

    fn y(&mut self) -> u32;

    fn mouse_inside(&mut self) -> bool;

    fn on_mouse_press(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_release(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_move(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_enter(&mut self) {
    }

    fn on_mouse_leave(&mut self) {
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

    pub fn mouse_enter(&mut self) {
        self.mouse_inside = true;
    }

    pub fn mouse_leave(&mut self) {
        self.mouse_inside = false;
    }
}
