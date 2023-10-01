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
    pub position: Vec2,
    pub cell_type: CellType,
    pub direction: Vec2,
}

impl Cell {
    pub fn render(&self) {
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
