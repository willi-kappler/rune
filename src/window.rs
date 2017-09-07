use sdl2;

use rune::{RuneAction, RuneMouseButton};
use widget::RuneWidget;

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
    pub w: u32,
    pub h: u32,
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

}

pub struct RuneWindowBuilder {
    title: String,
    w: u32,
    h: u32,
    event_handler: Box<RuneWindowHandler>,
}

impl RuneWindowBuilder {
    pub fn new(title: &str, w: u32, h: u32) -> RuneWindowBuilder {
        RuneWindowBuilder {
            title: title.to_string(),
            w,
            h,
            event_handler: Box::new(DefaultHandler {}),
        }
    }

    pub fn on_close_quit(self) -> RuneWindowBuilder {
        RuneWindowBuilder {
            title: self.title,
            w: self.w,
            h: self.h,
            event_handler: Box::new(CloseWindowHandler {}),
        }
    }

    pub fn set_event_handler<T>(self, event_handler: T) -> RuneWindowBuilder where T: 'static + RuneWindowHandler {
        RuneWindowBuilder {
            title: self.title,
            w: self.w,
            h: self.h,
            event_handler: Box::new(event_handler),
        }
    }

    pub fn build(self) -> RuneWindow {
        RuneWindow{
            title: self.title,
            w: self.w,
            h: self.h,
            widgets: Vec::new(),
            event_handler: self.event_handler,
        }
    }
}

pub struct RuneWindowInternal {
    pub rune_window: RuneWindow,
    pub id: u32,
    pub sdl_window: sdl2::video::Window,
}
