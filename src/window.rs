use sdl2;

use rune::{RuneAction, RuneMouseButton};
use widget::RuneWidget;
use canvas::RuneCanvas;

pub trait RuneWindowHandler {
    fn on_close(&mut self) -> Option<RuneAction> {
        Some(RuneAction::WindowHide)
    }

    fn on_move(&mut self, _: i32, _: i32) -> Option<RuneAction> {
        None
    }

    fn on_resize(&mut self, _: i32, _: i32) -> Option<RuneAction> {
        None
    }

    fn on_minimize(&mut self) -> Option<RuneAction> {
        None
    }

    fn on_maximize(&mut self) -> Option<RuneAction> {
        None
    }

    fn on_enter(&mut self) -> Option<RuneAction> {
        None
    }

    fn on_leave(&mut self) -> Option<RuneAction> {
        None
    }
}

struct DefaultHandler;

impl RuneWindowHandler for DefaultHandler {

}

struct CloseWindowHandler;

impl RuneWindowHandler for CloseWindowHandler {
    fn on_close(&mut self) -> Option<RuneAction> {
        Some(RuneAction::ApplicationQuit)
    }
}

pub struct RuneWindow {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub widgets: Vec<Box<RuneWidget>>,
    pub event_handler: Box<RuneWindowHandler>,
}

impl RuneWindow {
    pub fn add_widget<T>(&mut self, widget: T) where T: 'static + RuneWidget {
        self.widgets.push(Box::new(widget));
    }
}

pub struct RuneWindowBuilder {
    title: String,
    width: u32,
    height: u32,
    event_handler: Box<RuneWindowHandler>,
}

impl RuneWindowBuilder {
    pub fn new(title: &str, width: u32, height: u32) -> RuneWindowBuilder {
        RuneWindowBuilder {
            title: title.to_string(),
            width,
            height,
            event_handler: Box::new(DefaultHandler {}),
        }
    }

    pub fn on_close_quit(self) -> RuneWindowBuilder {
        RuneWindowBuilder {
            event_handler: Box::new(CloseWindowHandler {}),
            .. self
        }
    }

    pub fn set_event_handler<T>(self, event_handler: T) -> RuneWindowBuilder where T: 'static + RuneWindowHandler {
        RuneWindowBuilder {
            event_handler: Box::new(event_handler),
            .. self
        }
    }

    pub fn build(self) -> RuneWindow {
        RuneWindow{
            title: self.title,
            width: self.width,
            height: self.height,
            widgets: Vec::new(),
            event_handler: self.event_handler,
        }
    }
}

pub struct RuneWindowInternal {
    pub rune_window: RuneWindow,
    pub id: u32,
    pub canvas: RuneCanvas,
}

impl RuneWindowInternal {
    pub fn draw(&mut self) {
        for widget in self.rune_window.widgets.iter_mut() {
            widget.draw(&mut self.canvas);
        }
    }

    pub fn mouse_press(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        for widget in self.rune_window.widgets.iter_mut() {
            if contains_point(widget, x, y) {
                let wx = widget.get_x();
                let wy = widget.get_y();

                return widget.on_mouse_press(mouse_button, x - wx, y - wy);
            }
        }

        None
    }

    pub fn mouse_release(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        for widget in self.rune_window.widgets.iter_mut() {
            if contains_point(widget, x, y) {
                let wx = widget.get_x();
                let wy = widget.get_y();

                return widget.on_mouse_release(mouse_button, x - wx, y - wy);
            }
        }

        None
    }

    pub fn mouse_move(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        for widget in self.rune_window.widgets.iter_mut() {
            if contains_point(widget, x, y) {
                let wx = widget.get_x();
                let wy = widget.get_y();

                if !widget.get_mouse_inside() {
                    widget.set_mouse_inside(true);
                    widget.on_mouse_enter();
                }
                return widget.on_mouse_move(mouse_button, x - wx, y - wy);
            } else {
                if widget.get_mouse_inside() {
                    widget.set_mouse_inside(false);
                    widget.on_mouse_leave();
                }
            }
        }

        None
    }

}

fn contains_point(widget: &Box<RuneWidget>, x: u32, y: u32) -> bool {
    let wx = widget.get_x();
    let wy = widget.get_y();
    let ww = widget.get_width();
    let wh = widget.get_height();

    (x >= wx && x <= (wx + ww)) && (y >= wy && y <= (wy + wh))
}
