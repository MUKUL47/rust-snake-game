use std::{ collections::{ btree_map::Range, HashSet }, path };

use rand::Rng;
use sdl2::{ keyboard::Keycode, pixels::Color, rect::{ Point, Rect }, sys::{ random, KeyCode } };
#[path = "../render/mod.rs"]
pub mod render;

use render::Entity;

#[derive(Clone)]
pub struct Food {
    pub coordinate: Point,
}
impl Entity for Food {
    fn draw<T>(&mut self, mut f: T) where T: FnMut(sdl2::rect::Rect) {
        f(Rect::new(self.coordinate.x, self.coordinate.y, 20, 20));
    }
    fn update(&mut self) {}

    fn get_color(&self) -> Color {
        Color::RGB(255, 0, 0)
    }
}
impl Food {
    pub fn new(x: i32, y: i32) -> Self {
        Food { coordinate: Point::new(x, y) }
    }
    fn update_coordinate(&mut self, x: i32, y: i32) -> &mut Self {
        self.coordinate = Point::new(x, y);
        self
    }

    pub fn update_at_random(&mut self, ignore_points: &Vec<Point>) -> &mut Self {
        let mut point_set: HashSet<String> = std::collections::HashSet::new();
        for p in ignore_points.into_iter() {
            point_set.insert(p.x.to_string() + &p.y.to_string());
        }
        loop {
            let (x, y) = Food::get_random_point();
            if !point_set.contains(&(x.to_string() + &y.to_string())) {
                self.update_coordinate(x, y);
                break;
            }
        }
        self
    }

    fn get_random_point() -> (i32, i32) {
        (
            (rand::thread_rng().gen_range(0..500) / 20) * 20,
            (rand::thread_rng().gen_range(0..500) / 20) * 20,
        )
    }
}
