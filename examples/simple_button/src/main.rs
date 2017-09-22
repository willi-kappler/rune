#[macro_use] extern crate error_chain;

extern crate rune;

use rune::{Rune, RuneWindowBuilder, PushButtonBuilder, RuneMessageHandler, RuneMessageBox, RuneMessage, Result};

struct Button1 {
    counter: u32,
}

impl RuneMessageHandler for Button1 {
    fn process_messages(&mut self, button_mb: &mut RuneMessageBox, parent: &mut RuneMessageBox) -> Result<()> {
        loop {
            if let Some((_, message)) = button_mb.pop_message()? {
                match message {
                    RuneMessage::ButtonClick => {
                        self.counter += 1;

                        if self.counter == 1 {
                            println!("Button pressed 1 time");
                        } else {
                            println!("Button pressed {} times", self.counter);
                        }
                    },
                    _ => {

                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}

struct Button2;

impl RuneMessageHandler for Button2 {
    fn process_messages(&mut self, button_mb: &mut RuneMessageBox, parent: &mut RuneMessageBox) -> Result<()> {
        loop {
            if let Some((_, message)) = button_mb.pop_message()? {
                match message {
                    RuneMessage::ButtonClick => {
                        parent.send_message(button_mb, RuneMessage::ApplicationQuit)?
                    },
                    _ => {

                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}

quick_main!(|| -> Result<()> {
    let mut app = Rune::init()?;

    let mut main_window = RuneWindowBuilder::new("Rune button window", 640, 480).on_close_quit().build();

    let button1 = PushButtonBuilder::new("Click me", 10, 10).set_event_handler(Button1{ counter: 0 }).build();
    let button2 = PushButtonBuilder::new("Quit", 10, 40).set_event_handler(Button2{}).build();

    main_window.add_widget(button1);
    main_window.add_widget(button2);

    app.add_window(main_window)?;

    app.run()
});
