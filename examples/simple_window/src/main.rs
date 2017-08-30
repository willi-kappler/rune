extern crate rune;

use rune::{Rune};

fn main() {
    let mut main_rune = Rune::init().unwrap();
    let window1 = main_rune.new_window("Rune window test 1", 640, 480);
    main_rune.run();
}

