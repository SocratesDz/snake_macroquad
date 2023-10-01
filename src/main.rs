mod app;
mod board;
mod cell;
mod constants;
mod snake;
mod theme;
mod utils;

use std::time;

use app::App;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    const SCREEN_WIDTH: i32 = 640;
    const SCREEN_HEIGHT: i32 = 480;

    Conf {
        window_title: String::from("Snake"),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(
        time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let mut app = App::new();

    loop {
        let delta = get_frame_time();

        app.update(delta);
        app.render();

        next_frame().await
    }
}
