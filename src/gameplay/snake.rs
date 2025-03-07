use termion::event::Key;

use crate::{
    cells::{
        cell::{Cell, CellType},
        cell_group::CellGroup,
        cell_matrix::CellMatrix,
        color::Color,
        vector::Vector,
    },
    INITIAL_SNAKE_LENGTH,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<Vector<u16>>,
    direction: Direction,
    speed: f32,
    movement_accumulator: f32,
    cell_group: CellGroup,
}

impl Snake {
    pub fn new(position: &Vector<u16>, speed: f32) -> Snake {
        let mut body = Vec::new();

        for i in 0..INITIAL_SNAKE_LENGTH {
            body.push(Vector::<u16>::new(position.x(), position.y() + i));
        }

        return Snake {
            body,
            direction: Direction::Up,
            speed,
            movement_accumulator: 0.0,
            cell_group: CellGroup::new(),
        };
    }

    pub fn none() -> Snake {
        return Snake {
            body: Vec::new(),
            direction: Direction::Up,
            speed: 0.0,
            movement_accumulator: 0.0,
            cell_group: CellGroup::new(),
        };
    }

    pub fn move_forward(&mut self, frame_duration: f64) -> Option<Vector<u16>> {
        self.movement_accumulator += frame_duration as f32;

        if self.movement_accumulator * self.speed >= 1.0 {
            self.movement_accumulator = 0.0;

            let new_head = self.new_head();

            self.body.insert(0, new_head.clone());
            self.cell_group
                .set_cell(self.body.pop().unwrap(), Cell::new_empty());

            return Some(new_head);
        }

        return None;
    }

    pub fn grow(&mut self) {
        let new_head = self.new_head();

        self.body.insert(0, new_head);
    }

    pub fn update(&mut self, pressed_key: Option<Key>) {
        match pressed_key {
            Some(Key::Up) => {
                if self.direction != Direction::Down {
                    self.change_direction(Direction::Up);
                }
            }
            Some(Key::Down) => {
                if self.direction != Direction::Up {
                    self.change_direction(Direction::Down);
                }
            }
            Some(Key::Left) => {
                if self.direction != Direction::Right {
                    self.change_direction(Direction::Left);
                }
            }
            Some(Key::Right) => {
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
        const PATTERNS: [[char; 4]; 2] = [['█', '▓', '▒', '░'], ['░', '▒', '▓', '█']];

        let mut reverse_pattern = false;
        let body_len = self.body.len();

        for i in 0..body_len {
            if i % 4 == 0 && i > 0 {
                reverse_pattern = !reverse_pattern;
            }

            let pattern_idx = reverse_pattern as usize;
            let char = PATTERNS[pattern_idx][i % 4];

            self.set_cell(self.body[i].clone(), char);
        }

        self.cell_group.render(cell_matrix);
    }

    fn new_head(&self) -> Vector<u16> {
        let head = self.body[0].clone();

        return match self.direction {
            Direction::Up => Vector::<u16>::new(head.x(), head.y() - 1),
            Direction::Down => Vector::<u16>::new(head.x(), head.y() + 1),
            Direction::Left => Vector::<u16>::new(head.x() - 1, head.y()),
            Direction::Right => Vector::<u16>::new(head.x() + 1, head.y()),
        };
    }

    fn set_cell(&mut self, position: Vector<u16>, char: char) {
        self.cell_group.set_cell(
            position,
            Cell::new(
                char,
                Color::Black.to_rgb(),
                Color::Green.to_rgb(),
                CellType::Snake,
            ),
        );
    }
}
