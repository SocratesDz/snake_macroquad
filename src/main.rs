use std::{collections::LinkedList, ops::Mul};

use macroquad::{miniquad::window::quit, prelude::*};

pub mod theme {
    use macroquad::prelude::Color;

    /**
     * bg1: #c7f0d8 rgb(199, 240, 216)
     * fg1: #778d90 rgb(119,141,144)
     * bg2: #43523d rgb(67,82,61)
     * fg2: #312b25 rgb(49,43,37)
     **/

    pub const BG1: Color = Color::new(0.78, 0.94, 0.84, 1.0);
    pub const FG1: Color = Color::new(0.46, 0.55, 0.56, 1.0);
    pub const BG2: Color = Color::new(0.26, 0.32, 0.24, 1.0);
    pub const FG2: Color = Color::new(0.19, 0.17, 0.14, 1.0);
}

const TILE_SIZE: f32 = 8.0;

struct Dot {
    position: Vec2,
    direction: Vec2,
}

impl Dot {
    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, TILE_SIZE, TILE_SIZE, theme::FG2)
    }
}

struct Snake {
    head: Dot,
    body: LinkedList<Dot>,
    speed: f32,
}

impl Snake {
    fn new(x: f32, y: f32) -> Snake {
        Snake {
            head: Dot {
                position: Vec2::new(x, y),
                direction: Vec2::new(-1.0, 0.0)
            },
            body: LinkedList::new(),
            speed: 1.0
        }
    }

    fn draw(&self) {
        self.head.draw();
        for dot in self.body.iter() {
            dot.draw()
        }
    }

    fn update(&mut self, dt: f32) {
        // Update head
        if self.head.direction.x == 0.0 {
            if is_key_pressed(KeyCode::A) {
                self.head.direction = Vec2::new(-1.0, 0.0)
            }
            if is_key_pressed(KeyCode::D) {
                self.head.direction = Vec2::new(1.0, 0.0)
            }
        }
        if self.head.direction.y == 0.0 {
            if is_key_pressed(KeyCode::W) {
                self.head.direction = Vec2::new(0.0, -1.0)
            }
            if is_key_pressed(KeyCode::S) {
                self.head.direction = Vec2::new(0.0, 1.0)
            }
        }
        self.head.position += dt * self.speed * self.head.direction * TILE_SIZE;

        // Update body segments
        for segment in self.body.iter_mut() {
            segment.position += dt * self.speed * segment.direction * TILE_SIZE;
        }
    }

    fn grow(&mut self) {
        let last_dot = self.body.back().unwrap_or(&self.head);
        
        self.body.push_back(
            Dot { 
                position: last_dot.position + last_dot.direction.mul(TILE_SIZE).mul(-1.0), 
                direction: last_dot.direction 
            }
        )
    }
}

fn draw_grid(x: f32, y: f32, width: f32, height: f32) {
    let tile_width_ammount = (width - x) / TILE_SIZE;
    let tile_height_ammount = (height - y) / TILE_SIZE;
    for i in 0..(tile_width_ammount as i32) {
        for j in 0..(tile_height_ammount as i32) {
            let x = x + (i as f32) * TILE_SIZE;
            let y = y + (j as f32) * TILE_SIZE;
            let w = TILE_SIZE;
            let h = TILE_SIZE;
            draw_rectangle_lines(x, y, w, h, 2.0, theme::BG1)
        }
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake::new(screen_width()/2.0, screen_height()/2.0);

    loop {
        let delta = get_frame_time();

        snake.update(delta);

        clear_background(theme::BG1);

        draw_grid(
            TILE_SIZE,
            TILE_SIZE,
            screen_width() - TILE_SIZE,
            screen_height() - TILE_SIZE,
        );

        snake.draw();

        if is_key_pressed(KeyCode::G) {
            snake.grow()
        }

        if is_key_released(KeyCode::Escape) {
            quit()
        }

        next_frame().await
    }
}
