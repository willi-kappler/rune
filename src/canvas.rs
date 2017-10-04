use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;


pub struct RuneColor {
    r: u8,
    g: u8,
    b: u8,
}

impl From<RuneColor> for pixels::Color {
    fn from(color: RuneColor) -> pixels::Color {
        pixels::Color::RGB(color.r, color.g, color.b)
    }
}

impl RuneColor {
    pub fn RGB(r: u8, g: u8, b: u8) -> RuneColor {
        RuneColor { r, g, b }
    }
}

pub struct RuneCanvas {
    pub sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    // TODO
}

impl RuneCanvas {
    pub fn set_fg_color(&mut self, color: RuneColor) {
        self.sdl_canvas.set_draw_color(color.into());
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.sdl_canvas.draw_rect(Rect::new(
            x as i32, y as i32, w, h
        ));
    }

    pub fn draw_text(&mut self, x: u32, y: u32, test: &str) {
    }
}
