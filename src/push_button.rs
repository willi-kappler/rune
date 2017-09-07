use sdl2::pixels;
use sdl2::rect::Rect;

use rune::{RuneAction, RuneMouseButton};
use widget::{RuneWidget, BaseWidget};
use canvas::RuneCanvas;

pub trait PushButtonHandler {
    fn on_click(&mut self) -> Option<RuneAction> {
        None
    }
}

struct DefaultHandler;

impl PushButtonHandler for DefaultHandler {
}

pub struct PushButton {
    base_widget: BaseWidget,
    text: String,
    pressed: bool,
    event_handler: Box<PushButtonHandler>,
}

impl RuneWidget for PushButton {
    fn contains_point(&mut self, x: u32, y: u32) -> bool {
        self.base_widget.contains_point(x, y)
    }

    fn x(&mut self) -> u32 {
        self.base_widget.x
    }

    fn y(&mut self) -> u32 {
        self.base_widget.y
    }

    fn mouse_inside(&mut self) -> bool {
        self.base_widget.mouse_inside
    }

    fn draw(&mut self, canvas: &mut RuneCanvas) {
        canvas.sdl_canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.sdl_canvas.draw_rect(Rect::new(
             self.base_widget.x as i32, self.base_widget.y as i32,
             self.base_widget.width, self.base_widget.height));
    }

    fn on_mouse_press(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        self.pressed = true;
        None
    }

    fn on_mouse_release(&mut self, mouse_button: RuneMouseButton, x: u32, y: u32) -> Option<RuneAction> {
        if self.pressed {
            self.pressed = false;
            (self.event_handler).on_click()
        } else {
            None
        }
    }

    fn on_mouse_enter(&mut self) {
        self.base_widget.mouse_enter();
    }

    fn on_mouse_leave(&mut self) {
        self.base_widget.mouse_leave();
        self.pressed = false;
    }
}

pub struct PushButtonBuilder {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    text: String,
    event_handler: Box<PushButtonHandler>,
 }



impl PushButtonBuilder {
    pub fn new(text: &str, x: u32, y: u32) -> PushButtonBuilder {
        PushButtonBuilder {
            x,
            y,
            width: 50,
            height: 10,
            text: text.to_string(),
            event_handler: Box::new(DefaultHandler{}),
        }
    }

    pub fn set_event_handler<T>(self, event_handler: T) -> PushButtonBuilder where T: 'static + PushButtonHandler {
        PushButtonBuilder {
            event_handler: Box::new(event_handler),
            .. self
        }
    }

    pub fn build(self) -> PushButton {
        PushButton {
            base_widget: BaseWidget {
                id: 0,
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height,
                mouse_inside: false,
            },
            text: self.text,
            pressed: false,
            event_handler: self.event_handler,
        }
    }
}
