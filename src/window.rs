use sdl2;

use widget::RuneWidget;

pub enum RuneWindowAction {
    Hide,
    Close,
    None
}

pub trait RuneWindowHandler {
    fn on_close(&mut self) -> RuneWindowAction {
        RuneWindowAction::Hide
    }
}

struct DefaultHandler;

impl RuneWindowHandler for DefaultHandler {

}

struct CloseWindowHandler;

impl RuneWindowHandler for CloseWindowHandler {
    fn on_close(&mut self) -> RuneWindowAction {
        RuneWindowAction::Close
    }
}

pub struct RuneWindow {
    pub title: String,
    pub w: u32,
    pub h: u32,
    pub widgets: Vec<Box<RuneWidget>>,
    pub event_handler: Box<RuneWindowHandler>,
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
