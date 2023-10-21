use super::{cell::{Cell, CellType}, constants::LEFT, };

pub struct Snake {
    pub head: Cell,
    pub body: Vec<Cell>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            head: Cell {
                x,
                y,
                cell_type: CellType::SnakeBody,
                direction: LEFT,
            },
            body: vec![],
        }
    }

    pub fn update(&mut self) {
        self.head.update();
        for cell in &mut self.body {
            cell.update();
        }
    }
}