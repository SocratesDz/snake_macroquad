use super::{board::Board, cell::CellType, snake::Snake};

pub const FOOD_BASE_VALUE: u32 = 1;

pub struct Game {
    // TODO: Add current state
    // TOD: Add queue of events
    pub board: Board,
    pub snake: Snake,
    pub score: u32,
}

impl Game {
    pub fn new(rows: u32, columns: u32) -> Self {
        Self {
            board: Board::new(rows, columns),
            snake: Snake::new((rows / 2) as i32, (columns / 2) as i32),
            score: 0,
        }
    }

    pub fn update(&mut self) {
        self.snake.update();
        // Check for snake collision
        if let Some((index, cell)) = self
            .snake
            .body
            .iter()
            .chain(self.board.cells.iter())
            .enumerate()
            .find(|(_, cell)| cell.position() == self.snake.head.position())
        {
            match cell.cell_type {
                CellType::SnakeBody | CellType::Wall => {
                    // Emit hit event
                    // Then emit game over state
                }
                CellType::Food => {
                    self.snake.grow();
                    
                    self.board.cells.remove(index);

                    self.generate_food();

                    self.score += FOOD_BASE_VALUE;
                }
                _ => {}
            }
        }
    }

    fn generate_food(&mut self) {
        self.board.generate_random_food(&self.snake)
    }
}
