use termion::event::Key;

use crate::{
    cells::{
        cell::{Cell, CellType},
        cell_group::{self, CellGroup},
        cell_matrix::CellMatrix,
        color::Color,
        vector::VectorU16,
    },
    FPS,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<(u16, u16)>,
    direction: Direction,
    speed: f32,
    movement_accumulator: f32,
    cell_group: CellGroup,
}

impl Snake {
    pub fn new(x: u16, y: u16, speed: f32) -> Snake {
        let mut cell_group = cell_group::CellGroup::new();

        cell_group.set_cell(
            VectorU16::new(x, y),
            Cell::new('█', Color::Green.to_rgb(), CellType::Snake),
        );

        Snake {
            body: vec![(x, y)],
            direction: Direction::Right,
            speed,
            movement_accumulator: 0.0,
            cell_group,
        }
    }

    pub fn move_forward(&mut self) -> Option<(u16, u16)> {
        self.movement_accumulator += self.speed / FPS as f32;

        if self.movement_accumulator >= 1.0 {
            self.movement_accumulator = 0.0;

            let new_head_position = self.new_head_position();

            self.body.insert(0, new_head_position);
            let tail = self.body.pop();

            self.cell_group.set_cell(
                VectorU16::new(new_head_position.0, new_head_position.1),
                Cell::new('█', Color::Green.to_rgb(), CellType::Snake),
            );
            self.cell_group.set_cell(
                VectorU16::new(tail.unwrap().0, tail.unwrap().1),
                Cell::new_empty(),
            );

            return Some(new_head_position);
        }

        return None;
    }

    pub fn grow(&mut self) {
        let new_head = self.new_head_position();

        self.body.insert(0, new_head);

        self.cell_group.set_cell(
            VectorU16::new(new_head.0, new_head.1),
            Cell::new('█', Color::Green.to_rgb(), CellType::Snake),
        );
    }

    pub fn update(&mut self, pressed_key: Option<Key>) {
        match pressed_key {
            Some(termion::event::Key::Up) => {
                if self.direction != Direction::Down {
                    self.change_direction(Direction::Up);
                }
            }
            Some(termion::event::Key::Down) => {
                if self.direction != Direction::Up {
                    self.change_direction(Direction::Down);
                }
            }
            Some(termion::event::Key::Left) => {
                if self.direction != Direction::Right {
                    self.change_direction(Direction::Left);
                }
            }
            Some(termion::event::Key::Right) => {
                if self.direction != Direction::Left {
                    self.change_direction(Direction::Right);
                }
            }
            _ => (),
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.cell_group.render(cell_matrix);
    }

    fn new_head_position(&self) -> (u16, u16) {
        let (head_x, head_y) = self.body[0];

        match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }
}
