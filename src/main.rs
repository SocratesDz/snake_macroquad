use std::{collections::HashMap, time};

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

struct Board {
    width: i32,
    height: i32,
    cells: HashMap<(i32, i32), Cell>,
}

impl Board {
    fn new(size: (i32, i32)) -> Self {
        Self {
            width: size.0,
            height: size.1,
            cells: HashMap::new(),
        }
    }

    fn render(&self) {
        for i in 0..self.width {
            for j in 0..self.height {
                draw_rectangle(
                    (i as f32) * TILE_SIZE,
                    (j as f32) * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                    theme::BG1,
                )
            }
        }

        for (_position, cell) in &self.cells {
            cell.render();
        }
    }

    fn insert(&mut self, position: Vec2, cell: Cell) {
        self.cells
            .insert((position.x as i32, position.y as i32), cell);
    }

    fn has_food(&self) -> bool {
        self.cells.values().any(|c| c.cell_type == CellType::Food)
    }

    fn get_cell(&self, position: Vec2) -> Option<&Cell> {
        self.cells.get(&(position.x as i32, position.y as i32))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Wall,
    SnakeBody,
    Food,
}

#[derive(Clone, Copy)]
struct Cell {
    position: Vec2,
    cell_type: CellType,
    direction: Vec2,
}

impl Cell {
    fn render(&self) {
        match self.cell_type {
            CellType::Food => draw_circle_lines(
                self.position.x * TILE_SIZE,
                self.position.y * TILE_SIZE,
                TILE_SIZE / 2.,
                1.0,
                theme::FG2,
            ),
            CellType::SnakeBody => draw_rectangle(
                self.position.x * TILE_SIZE,
                self.position.y * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                theme::FG2,
            ),
            _ => {}
        }
    }
}

struct Snake {
    head: Cell,
    body: Vec<Cell>,
    speed: f32,
    last_update: f64,
    is_hit: bool,
    navigation_lock: bool,
}

impl Snake {
    fn new() -> Self {
        Self {
            head: Cell {
                position: Vec2::new(10., 10.),
                cell_type: CellType::SnakeBody,
                direction: Vec2::new(-1., 0.),
            },
            body: vec![],
            speed: 1.0,
            last_update: get_time(),
            is_hit: false,
            navigation_lock: false,
        }
    }

    fn update(&mut self, dt: f32, board: &mut Board) {
        // Player controller
        if !self.navigation_lock {
            if self.head.direction.x == 0. {
                if is_key_pressed(KeyCode::A) {
                    self.head.direction = Vec2::new(-1., 0.);
                    self.navigation_lock = true;
                }
                if is_key_pressed(KeyCode::D) {
                    self.head.direction = Vec2::new(1., 0.);
                    self.navigation_lock = true;
                }
            }
            if self.head.direction.y == 0. {
                if is_key_pressed(KeyCode::W) {
                    self.head.direction = Vec2::new(0., -1.);
                    self.navigation_lock = true;
                }
                if is_key_pressed(KeyCode::S) {
                    self.head.direction = Vec2::new(0., 1.);
                    self.navigation_lock = true;
                }
            }
        }

        // Update movement
        if get_time() - self.last_update > self.speed as f64 {
            self.navigation_lock = false;

            if !board.has_food() {
                generate_food(board)
            }
            self.last_update = get_time();
            self.body.insert(0, self.head);
            self.head.position += self.head.direction;
            if let Some(cell) = board.get_cell(self.head.position) {
                match cell.cell_type {
                    CellType::Food => {
                        self.grow();
                        clear_cell(self.head.position, board);
                        self.speed *= 0.9
                    }
                    _ => {}
                }
            }
            self.body.pop();

            for c in &self.body {
                if self.head.position == c.position {
                    self.is_hit = true;
                }
            }
        }
    }

    fn render(&self) {
        self.head.render();

        for cell in &self.body {
            cell.render()
        }
    }

    fn grow(&mut self) {
        self.body.push(self.head)
    }
}

struct App {
    board: Board,
    snake: Snake,
    game_over: bool,
}

impl App {
    fn new() -> Self {
        Self {
            board: Board::new((100, 100)),
            snake: Snake::new(),
            game_over: false,
        }
    }

    fn update(&mut self, dt: f32) {
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

    fn render(&self) {
        clear_background(theme::BG1);
        if self.game_over {
            draw_text(
                "GAME OVER",
                screen_width() / 2.,
                screen_height() / 2.,
                12.,
                theme::FG2,
            )
        } else {
            self.board.render();
            self.snake.render();
        }
    }

    fn reset(&mut self) {
        self.board = Board::new((100, 100));
        self.snake = Snake::new();
        self.game_over = false;
    }
}

fn generate_food(board: &mut Board) {
    let position = Vec2::new(
        rand::gen_range(0., board.width as f32),
        rand::gen_range(0., board.height as f32),
    );
    let cell = Cell {
        position: position,
        cell_type: CellType::Food,
        direction: Vec2::ZERO,
    };
    board.insert(position, cell);
}

fn clear_cell(position: Vec2, board: &mut Board) {
    board
        .cells
        .remove_entry(&(position.x as i32, position.y as i32));
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Snake"),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_secs());

    let mut app = App::new();

    loop {
        let delta = get_frame_time();

        app.update(delta);
        app.render();

        next_frame().await
    }
}
