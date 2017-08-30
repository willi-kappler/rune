use std::rc::Rc;
use std::cell::RefCell;

use sdl2;

use widget::{Widget, new_widget};

pub struct RuneWindow {
    pub rune_window: Rc<RefCell<RuneWindowInternal>>,
}

pub struct RuneWindowInternal {
    pub title: String,
    pub sdl_window: sdl2::video::Window,
    pub widget: Widget,
}
