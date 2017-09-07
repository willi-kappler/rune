
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
     fn on_mouse_press(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
         None
     }

     fn on_mouse_release(&mut self, mouse_button: RuneMouseButton, x: i32, y: i32) -> Option<RuneAction> {
         None
     }

     fn draw(&mut self, canvas: &mut RuneCanvas) {
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
             width: 0,
             height: 0,
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
             },
             text: self.text,
             pressed: false,
             event_handler: self.event_handler,
         }
     }
 }
