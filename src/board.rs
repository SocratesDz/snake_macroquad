use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    cell::{Cell, CellType},
    constants::TILE_SIZE,
    theme,
};

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub cells: HashMap<(i32, i32), Cell>,
}

impl Board {
    pub fn new(size: (i32, i32)) -> Self {
        Self {
            width: size.0,
            height: size.1,
            cells: HashMap::new(),
        }
    }

    pub fn render(&self) {
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

        self.cells.iter().for_each(|(_position, cell)| {
            cell.render();
        });
    }

    pub fn insert(&mut self, position: IVec2, cell: Cell) {
        self.cells.insert((position.x, position.y), cell);
    }

    pub fn has_food(&self) -> bool {
        self.cells.values().any(|c| c.cell_type == CellType::Food)
    }

    pub fn get_cell(&self, position: IVec2) -> Option<&Cell> {
        self.cells.get(&(position.x, position.y))
    }

    pub fn draw_debug_lines(&self) {
        for i in 0..self.width {
            for j in 0..self.height {
                draw_rectangle_lines(
                    (i as f32) * TILE_SIZE,
                    (j as f32) * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                    1.,
                    GREEN,
                )
            }
        }
    }
}
