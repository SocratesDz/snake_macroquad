pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub cell_type: CellType,
    pub direction: (i32, i32),
}

impl Cell {
    pub fn update(&mut self) {
        self.x += self.direction.0;
        self.y += self.direction.1;
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

pub enum CellType {
    Wall,
    SnakeBody,
    Food,
    Empty,
}