use sdl2::mouse::{MouseButton};

pub enum RuneEvent {
    MouseEvent(RuneMouseState, RuneMouseButton, i32, i32),

    Custom(RuneCustomType),
    None
}

pub enum RuneAction {
    ApplicationQuit,
    HideWindow(u32),

    Custom(RuneCustomType),
    None
}

pub enum RuneCustomType {
    Rbool(bool),
    Ru64(u64),
    Rf64(f64),
    RString(String)
}

pub enum RuneMouseState {
    Press,
    Release,
    Move
}

pub enum RuneMouseButton {
    Left,
    Middle,
    Right,
    Unknown
}

impl From<MouseButton> for RuneMouseButton {
    fn from (btn: MouseButton) -> Self {
        match btn {
            MouseButton::Left => {
                RuneMouseButton::Left
            },
            MouseButton::Middle => {
                RuneMouseButton::Middle
            },
            MouseButton::Right => {
                RuneMouseButton::Right
            },
            _ => {
                RuneMouseButton::Unknown
            }
        }
    }
}
