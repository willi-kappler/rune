use sdl2::mouse::MouseButton;

#[derive(Clone, Copy, Debug)]
pub enum RuneMouseButton {
    Left,
    Middle,
    Right,
    Unknown
}

impl From<MouseButton> for RuneMouseButton {
    fn from(btn: MouseButton) -> RuneMouseButton {
        match btn {
            MouseButton::Left => RuneMouseButton::Left,
            MouseButton::Middle => RuneMouseButton::Middle,
            MouseButton::Right => RuneMouseButton::Right,
            _ => RuneMouseButton::Unknown
        }
    }
}
