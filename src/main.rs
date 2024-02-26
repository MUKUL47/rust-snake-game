extern crate sdl2;

use entity::food::Food;
use entity::snake::Snake;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::Canvas;
use std::time::Duration;
mod entity;
mod render;
#[path = "./render/mod.rs"]
mod entities;
use crate::entity::snake::render::Entity;
use crate::entity::food::render::Entity as EE;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Snake", 500, 500).position_centered().build().unwrap();

    let mut canvas: Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut food: Food = Food::new(100, 100);
    let mut snake = Snake::new(0, 0);
    'main_loop: loop {
        canvas.set_draw_color(food.get_color());
        food.draw(|rect| canvas.fill_rect(rect).unwrap());
        canvas.set_draw_color(snake.get_color());
        snake.draw(|rect| canvas.fill_rect(rect).unwrap());
        if snake.found_food(food.coordinate) {
            food.update_at_random(&snake.body);
        }
        snake.update();
        if snake.is_head_collided() {
            break 'main_loop;
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop;
                }
                Event::KeyDown { keycode, .. } => {
                    snake.change_direction(keycode.unwrap());
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::new(0, 15_000_0000));
    }
}
