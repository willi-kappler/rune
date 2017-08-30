use sdl2;

use widget::{Widget, new_widget};

pub struct Window {
    pub title: String,
    pub sdl_window: sdl2::video::Window,
    pub widget: Widget,
}
