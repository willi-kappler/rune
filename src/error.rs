use sdl2;
use std;

error_chain! {
    foreign_links {
        WindowError(sdl2::video::WindowBuildError);
        SDLError(sdl2::IntegerOrSdlError);
        MutexError(std::sync::PoisonError<T>);
    }
}
