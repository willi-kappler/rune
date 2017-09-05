extern crate rune;

use rune::{Rune, RuneEvent, RuneAction};

fn main() {
    let mut main_rune = Rune::init().unwrap();
    let mut window1 = main_rune.new_window("Rune window test 1", 640, 480).unwrap();
    let window2 = main_rune.new_window("Rune window test 2", 640, 480).unwrap();

    window1.process_event(|e|
        match e {
            RuneEvent::WindowCloseEvent => {
                RuneAction::ApplicationQuit
            },
            _ => {
                RuneAction::None
            }
        }
    );

    main_rune.run();
}

