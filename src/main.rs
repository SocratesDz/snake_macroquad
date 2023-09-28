use macroquad::{prelude::*, miniquad::window::quit};

struct Dot {
    x: f64,
    y: f64,
}

impl Dot {
    fn new(x: f64, y:f64) -> Dot {
        Dot { x, y }
    }
}

#[macroquad::main("Snake")]
async fn main() {
    loop {
        clear_background(BLACK);

        if is_key_released(KeyCode::Escape) {
            quit()
        }

        next_frame().await
    }
}
