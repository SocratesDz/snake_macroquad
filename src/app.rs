use std::ops::Div;

use macroquad::{miniquad::window::quit, prelude::*};

use crate::{
    board::Board,
    cell::{Cell, CellType},
    constants::{TILE_SIZE, VIRTUAL_HEIGHT, VIRTUAL_WIDTH},
    snake::Snake,
    theme,
    utils::get_render_scale,
};

pub struct App {
    pub board: Board,
    pub snake: Snake,
    pub game_over: bool,
    pub camera: Camera2D,
    pub render_target: RenderTarget,
}

impl App {
    pub fn new() -> Self {
        let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Nearest);

        let mut camera =
            Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
        camera.render_target = Some(render_target.clone());
        
        let (width, height) = (
                VIRTUAL_WIDTH.div(TILE_SIZE) as i32,
                VIRTUAL_HEIGHT.div(TILE_SIZE) as i32,
            );
        let board = Board::new((width, height));

        Self {
            board: board,
            snake: Snake::new(width/2, height/2),
            game_over: false,
            camera,
            render_target,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if is_key_released(KeyCode::Escape) {
            quit();
        }
        if self.game_over {
            if is_key_released(KeyCode::Enter) {
                self.reset();
            }
        } else {
            self.snake.update(dt, &mut self.board);
            if self.snake.is_hit {
                self.game_over = true;
            }
        }
    }

    pub fn render(&self) {
        set_camera(&self.camera);

        clear_background(theme::BG1);

        if self.game_over {
            let game_over_str = "GAME OVER";

            draw_text(
                game_over_str,
                (VIRTUAL_WIDTH / 2.) - (game_over_str.len()*16/4) as f32,
                VIRTUAL_HEIGHT / 2.,
                16.,
                theme::FG2,
            );
        } else {
            self.board.render();
            self.snake.render();
        }

        self.render_buffer()
    }

    pub fn render_buffer(&self) {
        set_default_camera();

        clear_background(BLACK);

        let scale = get_render_scale();
        draw_texture_ex(
            &self.render_target.texture,
            (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5,
            (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                flip_y: true,
                ..Default::default()
            },
        )
    }

    pub fn reset(&mut self) {
        self.board = Board::new((
            VIRTUAL_WIDTH.div(TILE_SIZE) as i32,
            VIRTUAL_HEIGHT.div(TILE_SIZE) as i32,
        ));
        self.snake = Snake::new(self.board.width/2, self.board.height/2);
        self.game_over = false;
    }
}

pub fn generate_food(board: &mut Board, snake: &Snake) {
    
    let mut position = ivec2(
        rand::gen_range::<i32>(0, board.width),
        rand::gen_range::<i32>(0, board.height),
    );
    // FIXME: Infinite loop if the user is too good
    while snake.body.iter().any(|c| c.grid_position == position) {
        position = ivec2(
                rand::gen_range::<i32>(0, board.width),
                rand::gen_range::<i32>(0, board.height),
            );
    }
    let cell = Cell {
        grid_position: position,
        cell_type: CellType::Food,
        direction: IVec2::ZERO,
    };
    board.insert(position, cell);
}

pub fn clear_cell(position: IVec2, board: &mut Board) {
    board.cells.remove_entry(&(position.x, position.y));
}
