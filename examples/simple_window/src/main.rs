#[macro_use] extern crate error_chain;

extern crate rune;

use rune::{Rune, RuneWindowBuilder, RuneMessageHandler, RuneMessageBox, RuneMessage, Result};

struct CustomWindow {
    counter: u32,
}

impl RuneMessageHandler for CustomWindow {
    fn process_messages(&mut self, window_mb: &mut RuneMessageBox, parent_mb: &mut RuneMessageBox) -> Result<()> {
        loop {
            if let Some((sender, message)) = window_mb.pop_message()? {
                match message {
                    RuneMessage::WindowClose(_) => {
                        self.counter += 1;

                        if self.counter == 1 {
                            println!("User clicked the close button 1 time");
                        } else {
                            println!("User clicked the close button {} times", self.counter);
                        }
                    },
                    RuneMessage::WindowMove(x, y) => {
                        println!("Window moved to: {}, {}", x, y);
                    },
                    RuneMessage::WindowResize(w, h) => {
                        println!("Window resized to: {}, {}", w, h);
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

    let win1 = RuneWindowBuilder::new("Rune window 1", 640, 480).on_close_quit().build();
    let win2 = RuneWindowBuilder::new("Rune window 2", 640, 480).build(); // Hide on close is default
    let win3 = RuneWindowBuilder::new("Rune window 3", 640, 480).set_event_handler(CustomWindow{ counter: 0 }).build();

    app.add_window(win1)?;
    app.add_window(win2)?;
    app.add_window(win3)?;

    app.run();

    Ok(())
});
