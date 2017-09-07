
use rune::{RuneAction, RuneMouseButton};
use canvas::RuneCanvas;

pub trait RuneWidget {
    fn on_mouse_press(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_release(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_move(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
        None
    }

    fn draw(&mut self, canvas: &mut RuneCanvas) {
        // Do nothing in the default implementation
    }
}

pub struct BaseWidget {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
