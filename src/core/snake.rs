use super::{cell::{Cell, CellType}, constants::LEFT, board::Board, };

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
        self.body.insert(0, self.head);
        self.head.update();
        self.body.pop();
    }

    pub fn grow(&mut self) {
        self.body.push(self.head)
    }

    pub fn change_direction(&mut self, direction: (i32, i32)) {
        self.head.direction = direction;
    }
}