use piston_window::*;

use crate::{COLS, GRID_SIZE, ROWS};

#[derive(Clone)]
pub struct Position(pub u32, pub u32);
pub struct Direction(pub i32, pub i32);

pub struct Snake {
    body: Vec<Position>,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![Position(COLS / 2, ROWS / 2)],
            direction: Direction(1, 0),
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        self.body.iter().for_each(|pos| {
            rectangle(
                [1.0; 4],
                [
                    (pos.0 * GRID_SIZE) as f64,
                    (pos.1 * GRID_SIZE) as f64,
                    GRID_SIZE as f64,
                    GRID_SIZE as f64,
                ],
                context.transform,
                graphics,
            );
        });
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn is_dead(&self) -> bool {
        if self.body.len() <= 1 {
            return false;
        }
        let head = &self.body[0];
        self.body
            .iter()
            .skip(1)
            .any(|pos| pos.0 == head.0 && pos.1 == head.1)
    }

    pub fn update(&mut self, food: &Position) -> bool {
        if self.body.len() == 0 {
            return false;
        }

        let mut x = (self.body[0].0 as i32) + self.direction.0;
        let mut y = (self.body[0].1 as i32) + self.direction.1;
        if x >= COLS as i32 {
            x = 0;
        }
        if y >= ROWS as i32 {
            y = 0;
        }
        if x < 0 {
            x = (COLS as i32) - 1;
        }
        if y < 0 {
            y = (ROWS as i32) - 1;
        }

        let x = x as u32;
        let y = y as u32;
        self.body.insert(0, Position(x, y));

        // If the head just ate food, don't remove the tail
        if x == food.0 && y == food.1 {
            return true;
        }

        self.body.pop();
        return false;
    }
}
