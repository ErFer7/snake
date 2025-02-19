use crate::{
    cell::{Cell, CellType, Color},
    cell_matrix::CellMatrix,
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
}

impl Snake {
    pub fn new(x: u16, y: u16, speed: f32) -> Snake {
        Snake {
            body: vec![(x, y)],
            direction: Direction::Right,
            speed,
            movement_accumulator: 0.0,
        }
    }

    pub fn move_forward(&mut self) -> Option<(u16, u16)> {
        self.movement_accumulator += self.speed / FPS as f32;

        if self.movement_accumulator >= 1.0 {
            self.movement_accumulator = 0.0;

            let new_head_position = self.new_head_position();

            self.body.insert(0, new_head_position);
            self.body.pop();

            return Some(new_head_position);
        }

        return None;
    }

    pub fn grow(&mut self) {
        let new_head = self.new_head_position();

        self.body.insert(0, new_head);
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn render(&self, cell_matrix: &mut CellMatrix) {
        for (x, y) in self.body.iter() {
            cell_matrix.set_cell(
                *x,
                *y,
                Cell::new('█', Color::Green.to_rgb(), CellType::Snake),
            );
        }
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
