use std::{ collections::btree_map::Range, path };

use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::{ Point, Rect },
    render::Canvas,
    sys::{ KeyCode, Window },
};

#[path = "../render/mod.rs"]
pub mod render;
#[path = "./food.rs"]
mod food;

use food::*;
use render::*;

use super::entities;

static BLOCK_SIZE: i32 = 20;
#[derive(Clone)]
pub struct Snake {
    pub body: Vec<Point>,
    direction: Keycode,
}
impl Entity for Snake {
    fn draw<T>(&mut self, mut f: T) where T: FnMut(Rect) {
        for p in self.body.iter() {
            f(Rect::new(p.x, p.y, 20, 20));
        }
    }
    fn update(&mut self) {
        self.move_in_direction();
    }

    fn get_color(&self) -> Color {
        Color::RGB(255, 255, 0)
    }
}
impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        Snake {
            body: vec![Point::new(x, y)],
            direction: Keycode::Right,
        }
    }
    pub fn update_body(&mut self, point: Point) -> &mut Self {
        self.body.push(*self.body.last().unwrap());
        self
    }
    pub fn move_in_direction(&mut self) {
        let new_head = self.get_next_node();
        self.body.push(new_head);
        self.body.remove(0);
    }

    fn get_next_node(&self) -> Point {
        let latest_node = self.body.last().unwrap();
        match self.direction {
            Keycode::Down => { Point::new(latest_node.x, latest_node.y + BLOCK_SIZE) }
            Keycode::Left => { Point::new(latest_node.x - BLOCK_SIZE, latest_node.y) }
            Keycode::Right => { Point::new(latest_node.x + BLOCK_SIZE, latest_node.y) }
            Keycode::Up => { Point::new(latest_node.x, latest_node.y - BLOCK_SIZE) }
            _ => { *latest_node }
        }
    }

    pub fn change_direction(&mut self, direction: Keycode) {
        if self.is_forbidden_direction(direction) {
            return;
        }
        self.direction = direction;
    }

    fn is_forbidden_direction(&self, next_direction: Keycode) -> bool {
        match next_direction {
            Keycode::Down if self.direction == Keycode::Up => {
                return true;
            }
            Keycode::Left if self.direction == Keycode::Right => {
                return true;
            }
            Keycode::Right if self.direction == Keycode::Left => {
                return true;
            }
            Keycode::Up if self.direction == Keycode::Down => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn get_head(&self) -> Option<&Point> {
        self.body.get(self.body.len() - 1)
    }

    pub fn found_food(&mut self, food: Point) -> bool {
        let head = self.get_head().unwrap();
        if head.x == food.x && head.y == food.y {
            self.update_body(food.clone());
            return true;
        }
        return false;
    }

    pub fn is_head_collided(&mut self) -> bool {
        let head = self.get_head().unwrap();
        for i in 1..self.body.len() - 1 {
            let v = self.body.get(i).unwrap();
            if head.x == v.x && head.y == v.y {
                return true;
            }
        }
        return false;
    }
}
