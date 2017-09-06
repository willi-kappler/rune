#[macro_use] extern crate error_chain;

extern crate rune;

use rune::{Rune, RuneWindowBuilder};

error_chain!{
    links {
        RuneError(rune::error::Error, rune::error::ErrorKind);
    }
}

quick_main!(|| -> Result<()> {
    let mut app = Rune::init()?;

    let win1 = RuneWindowBuilder::new("Rune window 1", 640, 480).on_close_quit().build();
    let win2 = RuneWindowBuilder::new("Rune window 2", 640, 480).build(); // Hide on close is default

    app.add_window(win1)?;
    app.add_window(win2)?;

    app.run();

    Ok(())
});
