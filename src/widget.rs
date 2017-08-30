use rand;

use canvas::Canvas;

pub struct Widget {
    id: u64,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    canvas: Canvas,
    active: bool,
    widgets: Vec<Box<WidgetTrait>>,
}

pub trait WidgetTrait {
    fn draw(&self) {}
    fn move_to(&self) {}
    fn resize(&self) {}
    fn add_child(&self, Box<WidgetTrait>) {}
    fn enable(&self, bool) {}
    fn process_event(&self) {}
}

pub fn new_widget(x: u32, y: u32, w: u32, h: u32) -> Widget  {
    Widget {
        id: rand::random::<u64>(),
        x: x,
        y: y,
        w: w,
        h: h,
        canvas: Canvas{},
        active: true,
        widgets: Vec::new(),
    }
}
