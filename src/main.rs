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

#[derive(Clone, Copy)]
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
    body: Vec<Dot>,
    speed: f32,
}

impl Snake {
    fn new(x: f32, y: f32) -> Snake {
        Snake {
            head: Dot {
                position: Vec2::new(x, y),
                direction: Vec2::new(-1.0, 0.0)
            },
            body: vec!(),
            speed: 1.0,
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


        // Change first body direction so it matches head.direction
        // Every node should change direction to the previous one.

        // Update body segments
        let mut previous_elem = Some(self.head.clone());
        for segment in &mut self.body {
            if let Some(previous) = previous_elem {
                let ghost_position = previous.position + (TILE_SIZE * previous.direction * -1.);
                if segment.direction != previous.direction && ghost_position.as_ivec2() == segment.position.as_ivec2() {
                    segment.direction = previous.direction;
                }
            }
            segment.position += dt * self.speed * segment.direction * TILE_SIZE;
            previous_elem = Some(segment.clone());
        }
    }

    fn grow(&mut self) {
        let last_dot = self.body.last().unwrap_or(&self.head);
        
        self.body.push(
            Dot { 
                position: last_dot.position + last_dot.direction * TILE_SIZE * -1., 
                direction: last_dot.direction 
            }
        )
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake::new(screen_width()/2.0, screen_height()/2.0);

    loop {
        let delta = get_frame_time();

        snake.update(delta);

        clear_background(theme::BG1);

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
