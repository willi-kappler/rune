use sdl2;

use rune::{RuneMouseButton};
use widget::RuneWidget;
use canvas::RuneCanvas;
use message::{RuneMessageHandler, DefaultMessageHandler, RuneMessage};
use error::{Result};

struct CloseWindowHandler;

impl RuneMessageHandler for CloseWindowHandler {
    fn process_messages(&mut self) -> Result<()> {
        self.parent.send_message(self.message_box, RuneMessage::ApplicationQuit)
    }
}

pub struct RuneWindow {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub message_box: RuneMessageBox,
    pub parent: RuneMessageBox,
    pub widgets: Vec<Box<RuneWidget>>,
    pub event_handler: Box<RuneMessageHandler>,
}

impl RuneWindow {
    pub fn add_widget<T>(&mut self, widget: T) where T: 'static + RuneWidget {
        self.widgets.push(Box::new(widget));
    }
}

pub struct RuneWindowBuilder {
    title: String,
    width: u32,
    height: u32,
    event_handler: Box<RuneMessageHandler>,
}

impl RuneWindowBuilder {
    pub fn new(title: &str, width: u32, height: u32) -> RuneWindowBuilder {
        RuneWindowBuilder {
            title: title.to_string(),
            width,
            height,
            event_handler: Box::new(DefaultMessageHandler {}),
        }
    }

    pub fn on_close_quit(self) -> RuneWindowBuilder {
        RuneWindowBuilder {
            event_handler: Box::new(CloseWindowHandler {}),
            .. self
        }
    }

    pub fn set_event_handler<T>(self, event_handler: T) -> RuneWindowBuilder where T: 'static + RuneMessageHandler {
        RuneWindowBuilder {
            event_handler: Box::new(event_handler),
            .. self
        }
    }

    pub fn build(self) -> RuneWindow {
        RuneWindow{
            title: self.title,
            width: self.width,
            height: self.height,
            message_box: RuneMessageBox::new(),
            parent: RuneMessageBox::new(),
            widgets: Vec::new(),
            event_handler: self.event_handler,
        }
    }
}

pub struct RuneWindowInternal {
    pub rune_window: RuneWindow,
    pub id: u32,
    pub canvas: RuneCanvas,
}

impl RuneWindowInternal {
    pub fn draw(&mut self) {
        for widget in self.rune_window.widgets.iter_mut() {
            widget.draw(&mut self.canvas);
        }
    }

    fn process_messages(&mut self) -> Result<()> {
        (self.rune_window.event_handler).process_messages()?
        // TODO: iterate over all messages, self.rune_window.message_box.pop_front()
        for widget in self.rune_window.widgets.iter_mut() {
            widget.send_message(self.rune_window.message_box, message)?
        }
    }

    fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        self.rune_window.message_box.send_message(sender, message)
    }
}
