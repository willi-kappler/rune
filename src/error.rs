use sdl2;

error_chain! {
    foreign_links {
        WindowError(sdl2::video::WindowBuildError);
        SDLError(sdl2::IntegerOrSdlError);
        SDLTTFError(sdl2::ttf::InitError);
    }
}
