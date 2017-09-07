
use rune::{RuneAction, RuneMouseButton};

pub trait RuneWidget {
    fn on_mouse_press(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_release(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
        None
    }

    fn draw(&mut self) {
        // Do nothing in the default implementation
    }
}
