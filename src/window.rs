use sdl2;

use mouse::{RuneMouseButton};
use widget::RuneWidget;
use canvas::RuneCanvas;
use message::{RuneMessageHandler, DefaultMessageHandler, RuneMessage, RuneMessageBox};
use error::{Result};

struct CloseWindowHandler;

impl RuneMessageHandler for CloseWindowHandler {
    fn process_messages(&mut self, window_mb: &mut RuneMessageBox, parent_mb: &mut RuneMessageBox) -> Result<()> {
        loop {
            if let Some((sender, message)) = window_mb.pop_message()? {
                match message {
                    RuneMessage::WindowClose => {
                        parent_mb.send_message(window_mb, &RuneMessage::ApplicationQuit)?;
                        break;
                    },
                    _ => {
                        // TODO: Nothing more for now
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
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
        widget.set_parent(self.message_box.clone());
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

    pub fn process_messages(&mut self) -> Result<()> {
        (self.rune_window.event_handler).process_messages(&mut self.rune_window.message_box, &mut self.rune_window.parent)?;
        for widget in self.rune_window.widgets.iter_mut() {
            widget.process_messages()?;
        }
        Ok(())
    }

    pub fn send_message(&mut self, sender: &RuneMessageBox, message: &RuneMessage) -> Result<()> {
        self.rune_window.message_box.send_message(sender, message)?;
        for widget in self.rune_window.widgets.iter_mut() {
            widget.send_message(sender, message)?;
        }
        Ok(())
    }
}
