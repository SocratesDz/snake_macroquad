use std::iter::once;

use macroquad::rand;

use super::cell::Cell;
use super::snake::Snake;

pub enum FoodError {
    NoEmptyCells,
    UsedCell,
}

pub struct Board {
    pub rows: u32,
    pub columns: u32,
    pub cells: Vec<Cell>,
}

impl Board {
    pub fn new(rows: u32, columns: u32) -> Self {
        Self {
            rows,
            columns,
            cells: vec![],
        }
    }

    pub fn place_food(&mut self, position: (u32, u32), snake: &Snake) -> Result<(), FoodError> {
        if snake.body.len() + 1 == (self.rows * self.columns) as usize {
            return Err(FoodError::NoEmptyCells);
        }
        if snake
            .body
            .iter()
            .chain(once(&snake.head))
            .chain(&self.cells)
            .any(|c| (c.x as u32, c.y as u32) == position)
        {
            return Err(FoodError::UsedCell);
        }
        let cell = Cell {
            x: position.0 as i32,
            y: position.1 as i32,
            cell_type: super::cell::CellType::Food,
            direction: (0, 0),
        };
        self.cells.push(cell);
        Ok(())
    }

    pub fn generate_random_food(&mut self, snake: &Snake) {
        let position = (
            rand::gen_range::<u32>(0, self.rows),
            rand::gen_range::<u32>(0, self.columns),
        );
        match self.place_food(position, snake) {
            Err(FoodError::UsedCell) => self.generate_random_food(snake),
            _ => {}
        }
    }
}
