use std::rc::Rc;
use std::cell::RefCell;

use sdl2::video::{Window};
use sdl2::event::{WindowEvent};
use sdl2::mouse::{MouseButton};

use widget::{Widget, new_widget};
use event::{RuneEvent, RuneAction, RuneMouseState, RuneMouseButton};

pub struct RuneWindow {
    pub rune_window: Rc<RefCell<RuneWindowInternal>>,
}

impl RuneWindow {
    pub fn process_event<F>(&mut self, event_handler: F) where F: 'static + FnMut(RuneEvent) -> (RuneAction) {
        self.rune_window.borrow_mut().event_handler = Box::new(event_handler);
    }

    pub fn on_close<F>(&mut self, event_handler: F) where F: 'static + FnMut() -> (RuneAction) {
        self.rune_window.borrow_mut().on_close = Box::new(event_handler);
    }
}

pub struct RuneWindowInternal {
    pub title: String,
    pub sdl_window: Window,
    pub widget: Widget,
    pub event_handler: Box<FnMut(RuneEvent) -> (RuneAction)>,
    pub on_close: Box<FnMut() -> (RuneAction)>,
}

impl RuneWindowInternal {
    pub fn process_window_event(&mut self, id: u32, event: WindowEvent) -> RuneAction {
        if id != self.widget.id {
            return RuneAction::None;
        }

        match event {
            WindowEvent::Close => {
                (self.on_close)()
            },
            _ => {
                // TODO: process more events...
                RuneAction::None
            }
        }
    }

    pub fn process_mouse_down_event(&mut self, id: u32, btn: MouseButton, x: i32, y: i32) -> RuneAction {
        if id != self.widget.id {
            return RuneAction::None;
        }

        (self.event_handler)(RuneEvent::MouseEvent(RuneMouseState::Press, RuneMouseButton::from(btn), x, y))
    }

    pub fn process_mouse_up_event(&mut self, id: u32, btn: MouseButton, x: i32, y: i32) -> RuneAction {
        if id != self.widget.id {
            return RuneAction::None;
        }

        (self.event_handler)(RuneEvent::MouseEvent(RuneMouseState::Release, RuneMouseButton::from(btn), x, y))
    }

    pub fn hide(&mut self, id: u32) {
        if id != self.widget.id {
            return;
        }

        self.sdl_window.hide();
    }
}
