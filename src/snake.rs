use std::ops::{Add, Rem};

use macroquad::prelude::*;

const INITIAL_SPEED: f32 = 1.;
const MAX_SPEED: f32 = 10.;
const SPEED_INCREASE: f32 = 1.5;

use crate::{
    app::{clear_cell, generate_food},
    board::Board,
    cell::{Cell, CellType},
};

pub struct Snake {
    pub head: Cell,
    pub body: Vec<Cell>,
    pub speed: f32,
    pub last_update: f64,
    pub is_hit: bool,
    navigation_lock: bool,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            head: Cell {
                grid_position: ivec2(2, 2),
                cell_type: CellType::SnakeBody,
                direction: ivec2(-1, 0),
            },
            body: vec![],
            speed: 1./INITIAL_SPEED,
            last_update: get_time(),
            is_hit: false,
            navigation_lock: false,
        }
    }

    pub fn update(&mut self, _dt: f32, board: &mut Board) {
        // Player controller
        if !self.navigation_lock {
            if self.head.direction.x == 0 {
                if is_key_pressed(KeyCode::A) {
                    self.head.direction = ivec2(-1, 0);
                    self.navigation_lock = true;
                }
                if is_key_pressed(KeyCode::D) {
                    self.head.direction = ivec2(1, 0);
                    self.navigation_lock = true;
                }
            }
            if self.head.direction.y == 0 {
                if is_key_pressed(KeyCode::W) {
                    self.head.direction = ivec2(0, -1);
                    self.navigation_lock = true;
                }
                if is_key_pressed(KeyCode::S) {
                    self.head.direction = ivec2(0, 1);
                    self.navigation_lock = true;
                }
            }
        }

        // Update movement
        if get_time() - self.last_update > self.speed as f64 {
            self.navigation_lock = false;

            if !board.has_food() {
                generate_food(board)
            }
            self.last_update = get_time();
            self.body.insert(0, self.head);
            self.head.grid_position += self.head.direction;

            // Warp
            self.head.grid_position = self
                .head
                .grid_position
                .add(ivec2(board.width, board.height))
                .rem(ivec2(board.width, board.height));

            // Check collisions
            if let Some(cell) = board.get_cell(self.head.grid_position) {
                match cell.cell_type {
                    CellType::Food => {
                        self.grow();
                        clear_cell(self.head.grid_position, board);
                        self.speed = clamp(
                            self.speed * (1. / SPEED_INCREASE),
                            1. / MAX_SPEED,
                            1. / INITIAL_SPEED,
                        );
                    }
                    CellType::Wall => {
                        self.is_hit = true;
                    }
                    _ => {}
                }
            }
            self.body.pop();

            for c in &self.body {
                if self.head.grid_position == c.grid_position {
                    self.is_hit = true;
                }
            }
        }
    }

    pub fn render(&self) {
        self.head.render();

        for cell in &self.body {
            cell.render()
        }
    }

    pub fn grow(&mut self) {
        self.body.push(self.head)
    }
}
