use macroquad::rand;

use super::cell::Cell;
use super::snake::Snake;

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
    }

    pub fn generate_food(&mut self) {
        if self.snake.body.len() >= (self.rows*self.columns) as usize {
            return;
        }
        let position = (
            rand::gen_range::<i32>(0, self.rows as i32),
            rand::gen_range::<i32>(0, self.columns as i32)
        );
        let cell = Cell {
            x: position.0,
            y: position.1,
            cell_type: super::cell::CellType::Food,
            direction: (0, 0)
        };
        self.cells.insert((position.0 + (self.rows as i32) * position.1) as usize, cell)
    }
}