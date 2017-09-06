use sdl2;
use error::{Result, ResultExt};

use window::{RuneWindow, RuneWindowInternal};

pub struct Rune {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
    windows: Vec<RuneWindowInternal>,
    quit: bool,
}

impl Rune {
    pub fn init() -> Result<Rune> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let event_pump = sdl_context.event_pump()?;

        Ok(
            Rune {
                sdl_context: sdl_context,
                video_subsystem: video_subsystem,
                event_pump: event_pump,
                windows: Vec::new(),
                quit: false,
            }
        )
    }

    pub fn add_window(&mut self, w: RuneWindow) -> Result<()> {
        let sdl_window = self.video_subsystem
            .window(&w.title, w.w, w.h)
            .resizable()
            .build()?;

        let window_id = sdl_window.id();

        self.windows.push(RuneWindowInternal{
            rune_window: w,
            id: window_id,
            sdl_window,
        });

        Ok(())
    }


    pub fn run(&mut self) {
        self.quit = false;

        let mut result_actions = Vec::new();

        while !self.quit {
            for event in self.event_pump.poll_iter() {
                for window in self.windows.iter() {
                    match event {
                        Event::Window { timestamp: _, window_id: id, win_event: we } => {
                            if window.id == id {
                                match we {

                                }
                            }
                        },
                        _ => {
                            // TODO: add more events...
                        }
                    }
                }
            }

            self.quit = true;
        }
    }

    fn process_events(&mut self, w: RuneWindowInternal, e: sdl2::event::Event) -> 


}
