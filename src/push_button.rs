use sdl2::pixels;
use sdl2::rect::Rect;

use message::{RuneMessageHandler, RuneMessageBox, RuneMessage, DefaultMessageHandler};
use widget::{RuneWidget, BaseWidget};
use canvas::RuneCanvas;
use error::{Result};

pub struct PushButton {
    base_widget: BaseWidget,
    text: String,
    pressed: bool,
    event_handler: Box<RuneMessageHandler>,
}

impl RuneWidget for PushButton {
    fn draw(&mut self, canvas: &mut RuneCanvas) {
        canvas.sdl_canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.sdl_canvas.draw_rect(Rect::new(
             self.base_widget.x as i32, self.base_widget.y as i32,
             self.base_widget.width, self.base_widget.height));
    }

    fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        self.message_box.send_message(sender, message)?;
        self.base_widget.send_message(sender, message)?
    }

    fn process_messages(&mut self) -> Result<()> {
        (self.event_handler).process_messages()?;
        self.base_widget.process_messages()?
    }
}

pub struct PushButtonBuilder {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    text: String,
    event_handler: Box<RuneMessageHandler>,
 }



impl PushButtonBuilder {
    pub fn new(text: &str, x: u32, y: u32) -> PushButtonBuilder {
        PushButtonBuilder {
            x,
            y,
            width: 50,
            height: 10,
            text: text.to_string(),
            event_handler: Box::new(DefaultMessageHandler{}),
        }
    }

    pub fn set_event_handler<T>(self, event_handler: T) -> PushButtonBuilder where T: 'static + RuneMessageHandler {
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
                message_box: RuneMessageBox::new(),
                parent: RuneMessageBox::new(),
            },
            text: self.text,
            pressed: false,
            event_handler: self.event_handler,
        }
    }
}
