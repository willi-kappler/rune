use std::sync::{Mutex};

lazy_static! {
    static ref APP_HANDLER: Mutex<Vec<MessageHandler>> = Mutex::new(Vec::new());
}

struct MessageHandler {
    id: String,
    messages: Vec<Message>,
    handler: Box<dyn Handler + Send>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub content: String,
}

pub trait Handler {
    fn handle(&mut self, message: Message) {
    }
}

fn add_handler<T: Handler + Send + 'static>(new_id: String, new_handler: T) {
    for handler in APP_HANDLER.lock().unwrap().iter() {
        if handler.id == new_id {
            // Handler already registered, nothing to do
            info!("Handler already registered: {}", new_id);
            return
        }
    }

    APP_HANDLER.lock().unwrap().push(
        MessageHandler {
            id: new_id,
            messages: Vec::new(),
            handler: Box::new(new_handler),
        }
    );
}

fn send_message(sender: String, receiver: String, content: String) {
    let new_message = Message {sender: sender.clone(), receiver: receiver.clone(), content};

    if sender == receiver {
        error!("Sending messages to self not allowed: {:?}", new_message);
        return
    }

    for mut handler in APP_HANDLER.lock().unwrap().iter_mut() {
        if handler.id == receiver {
            handler.messages.push(new_message.clone());
            debug!("Message queued: {:?}", new_message);
            return
        }
    }

    debug!("Receiver not found: {:?}", new_message);
}

fn process_messages() {
}

fn receive_message(sender: String, receiver: String) -> Option<String> {
    for mut handler in APP_HANDLER.lock().unwrap().iter_mut() {
        // TODO
    }

    None
}

pub fn new_application<T: Handler + Send + 'static>(app_id: String, app_handler: T) {
    add_handler(app_id.clone(), app_handler);
    send_message("RUNE".to_string(), app_id.clone(), "get_initial_properties".to_string());
    process_messages();

    let initial_properties = match receive_message(app_id, "RUNE".to_string()) {
        Some(properties) => {
            0
        }
        None => {
            0
        }
    };
}
