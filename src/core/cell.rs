use super::constants::Direction;

#[derive(Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub cell_type: CellType,
    pub direction: Direction,
}

impl Cell {
    pub fn update(&mut self) {
        self.x += self.direction.0;
        self.y += self.direction.1;
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            cell_type: CellType::Empty,
            direction: (0, 0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Wall,
    SnakeBody,
    Food,
    Empty,
}