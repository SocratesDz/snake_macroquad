use macroquad::window::{screen_height, screen_width};

use crate::constants::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};

pub fn get_render_scale() -> f32 {
    f32::min(
        screen_width() / VIRTUAL_WIDTH,
        screen_height() / VIRTUAL_HEIGHT,
    )
}
