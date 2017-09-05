pub enum RuneEvent {
    WindowCloseEvent,

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
