use canvas::Canvas;

pub struct Widget {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub canvas: Canvas,
    pub active: bool,
    pub widgets: Vec<Box<WidgetTrait>>,
}

pub trait WidgetTrait {
    fn draw(&self) {}
    fn move_to(&self) {}
    fn resize(&self) {}
    fn add_child(&self, Box<WidgetTrait>) {}
    fn enable(&self, bool) {}
    fn process_event(&self) {}
}

pub fn new_widget(id: u32, x: u32, y: u32, w: u32, h: u32) -> Widget  {
    Widget {
        id: id,
        x: x,
        y: y,
        w: w,
        h: h,
        canvas: Canvas{},
        active: true,
        widgets: Vec::new(),
    }
}
