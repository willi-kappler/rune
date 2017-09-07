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
    fn mouse_press(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
        None
    }

    fn mouse_release(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
        None
    }

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
}
