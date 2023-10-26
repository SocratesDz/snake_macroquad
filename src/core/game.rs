use super::{board::Board, cell::CellType, snake::Snake};

pub struct Game {
    // TODO: Add current state
    // TOD: Add queue of events
    pub board: Board,
    pub snake: Snake,
}

impl Game {
    pub fn new(rows: u32, columns: u32) -> Self {
        Self {
            board: Board::new(rows, columns),
            snake: Snake::new((rows / 2) as i32, (columns / 2) as i32),
        }
    }

    pub fn update(&mut self) {
        self.snake.update();
        // Check for snake collision
        if self
            .snake
            .body
            .iter()
            .chain(
                self.board
                    .cells
                    .iter()
                    .filter(|c| c.cell_type == CellType::Wall),
            )
            .any(|c| c.position() == self.snake.head.position())
        {
            // Snake collided
        }
        // Check for snake points
    }

    fn generate_food(&mut self) {
        self.board.generate_random_food(&self.snake)
    }
}
