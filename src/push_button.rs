use rune::{RuneAction, RuneMouseButton};
use widget::{RuneWidget};
use canvas::{RuneCanvas, RuneColor};

pub trait PushButtonHandler {
    fn on_click(&mut self) -> Option<RuneAction> {
        None
    }
}

struct DefaultHandler;

impl PushButtonHandler for DefaultHandler {
}

pub struct PushButton {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    mouse_inside: bool,
    text: String,
    pressed: bool,
    event_handler: Box<PushButtonHandler>,
}

impl RuneWidget for PushButton {
    fn get_x(&self) -> u32 {
        self.x
    }

    fn set_x(&mut self, x: u32) {
        self.x = x
    }

    fn get_y(&self) -> u32 {
        self.y
    }

    fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    fn set_mouse_inside(&mut self, mouse_inside: bool) {
        self.mouse_inside = mouse_inside;
    }

    fn get_mouse_inside(&self) -> bool {
        self.mouse_inside
    }

    fn draw(&mut self, canvas: &mut RuneCanvas) {
        canvas.set_fg_color(RuneColor::RGB(255, 255, 255));
        canvas.draw_rect(self.x, self.y, self.width, self.height);

//        canvas.sdl_canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
//        canvas.sdl_canvas.draw_rect(Rect::new(
//             self.x as i32, self.y as i32,
//             self.width, self.height));
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

    fn on_mouse_leave(&mut self) -> Option<RuneAction> {
        self.pressed = false;
        None
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
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            mouse_inside: false,
            text: self.text,
            pressed: false,
            event_handler: self.event_handler,
        }
    }
}
