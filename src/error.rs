use sdl2;

error_chain! {
    foreign_links {
        WindowError(sdl2::video::WindowBuildError);
    }
}
