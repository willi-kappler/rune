use std::rc::Rc;
use std::cell::RefCell;

use sdl2::video::{Window};
use sdl2::event::{WindowEvent};

use widget::{Widget, new_widget};
use event::{RuneEvent, RuneAction};

pub struct RuneWindow {
    pub rune_window: Rc<RefCell<RuneWindowInternal>>,
}

impl RuneWindow {
    pub fn process_event<F>(&mut self, event_handler: F) where F: 'static + FnMut(RuneEvent) -> (RuneAction) {
        self.rune_window.borrow_mut().event_handler = Box::new(event_handler);
    }
}

pub struct RuneWindowInternal {
    pub title: String,
    pub sdl_window: Window,
    pub widget: Widget,
    pub event_handler: Box<FnMut(RuneEvent) -> (RuneAction)>,
}

impl RuneWindowInternal {
    pub fn process_event(&mut self, event: WindowEvent) -> RuneAction {
        match event {
            WindowEvent::Close => {
                (self.event_handler)(RuneEvent::WindowCloseEvent)
            },
            _ => {
                // TODO: process more events...
                RuneAction::None
            }
        }
    }

    pub fn hide(&mut self) {
        self.sdl_window.hide();
    }
}