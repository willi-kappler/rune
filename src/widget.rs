
use rune::{RuneAction, RuneMouseButton};
use canvas::RuneCanvas;

pub trait RuneWidget {
    fn draw(&mut self, canvas: &mut RuneCanvas) {
        // Do nothing in the default implementation
    }

    fn get_x(&self) -> u32;

    fn set_x(&mut self, x: u32);

    fn get_y(&self) -> u32;

    fn set_y(&mut self, y: u32);

    fn get_width(&self) -> u32;

    fn set_width(&mut self, width: u32);

    fn get_height(&self) -> u32;

    fn set_height(&mut self, height: u32);

    fn get_mouse_inside(&self) -> bool;

    fn set_mouse_inside(&mut self, mouse_inside: bool);

    fn on_mouse_press(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_release(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_move(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        None
    }

    fn on_mouse_enter(&mut self) -> Option<RuneAction> {
        None
    }

    fn on_mouse_leave(&mut self) -> Option<RuneAction> {
        None
    }

}
