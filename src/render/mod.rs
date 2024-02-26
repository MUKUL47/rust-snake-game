use sdl2::{ pixels::Color, rect::Rect };

pub trait Entity {
    fn draw<T>(&mut self, f: T) where T: FnMut(Rect);
    fn update(&mut self);
    fn get_color(&self) -> Color;
}
