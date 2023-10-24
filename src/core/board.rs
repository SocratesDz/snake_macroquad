use std::iter::once;

use macroquad::rand;

use super::cell::Cell;
use super::snake::Snake;

pub enum FoodError {
    NoEmptyCells,
    UsedCell
}

pub struct Board {
    pub rows: u32,
    pub columns: u32,
    pub cells: Vec<Cell>,
    pub snake: Snake,
}

impl Board {
    pub fn new(rows: u32, columns: u32) -> Self {
        let mut cells = vec![];
        for r in 0..rows {
            for c in 0..columns {
                cells.push(Cell {
                    x: r as i32,
                    y: c as i32,
                    ..Default::default()
                })
            }
        }
        Self {
            rows,
            columns,
            cells,
            snake: Snake::new((rows/2) as i32, (columns/2) as i32),
        }
    }

    pub fn update(&mut self) {
        self.snake.update()

        // Check if snake collided
    }

    pub fn place_food(&mut self, position: (u32, u32)) -> Result<(), FoodError> {
        if self.snake.body.len() + 1 == (self.rows * self.columns) as usize {
            return Err(FoodError::NoEmptyCells)
        }
        if self.snake.body.iter().chain(once(&self.snake.head)).any(|c| (c.x as u32, c.y as u32) == position) {
            return Err(FoodError::UsedCell)
        }
        let cell = Cell {
            x: position.0 as i32, 
            y: position.1 as i32, 
            cell_type: super::cell::CellType::Food,
            direction: (0, 0)
        };
        self.cells.insert((position.0 + (self.rows) * position.1) as usize, cell);
        Ok(())
    }

    pub fn generate_random_food(&mut self) {
        let position = (
            rand::gen_range::<u32>(0, self.rows),
            rand::gen_range::<u32>(0, self.columns)
        );
        match self.place_food(position) {
            Err(FoodError::UsedCell) => {
                self.generate_random_food()
            },
            _ => {}
        }
    }
}