use macroquad::prelude::*;

use crate::{constants::TILE_SIZE, theme};

#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Wall,
    SnakeBody,
    Food,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub grid_position: IVec2,
    pub cell_type: CellType,
    pub direction: IVec2,
}

impl Cell {
    pub fn render(&self) {
        match self.cell_type {
            CellType::Food => draw_circle_lines(
                self.grid_position.x as f32 * TILE_SIZE + (TILE_SIZE / 2.),
                self.grid_position.y as f32 * TILE_SIZE + (TILE_SIZE / 2.),
                TILE_SIZE / 2.,
                1.0,
                theme::FG2,
            ),
            CellType::Wall | CellType::SnakeBody => draw_rectangle(
                self.grid_position.x as f32 * TILE_SIZE,
                self.grid_position.y as f32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                theme::FG2,
            ),
        }
    }
}
