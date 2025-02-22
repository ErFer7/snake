use termion::{
    color::{Fg, Reset, Rgb},
    cursor::Goto,
};

use super::{color::Color, vector::VectorU16};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Solid,
    Fruit,
    Snake,
    Empty,
}

#[derive(Clone)]
pub struct Cell {
    char: char,
    color: Rgb,
    cell_type: CellType,
}

impl Cell {
    pub fn new(char: char, color: Rgb, cell_type: CellType) -> Cell {
        return Cell {
            char,
            color,
            cell_type,
        };
    }

    pub fn new_typeless(char: char, color: Rgb) -> Cell {
        return Cell {
            char,
            color,
            cell_type: CellType::Solid,
        };
    }

    pub fn new_colorless(char: char) -> Cell {
        return Cell {
            char,
            color: Color::White.to_rgb(),
            cell_type: CellType::Solid,
        };
    }

    pub fn new_empty() -> Cell {
        Cell {
            char: ' ',
            color: Color::White.to_rgb(),
            cell_type: CellType::Empty,
        }
    }

    pub fn cell_type(&self) -> CellType {
        return self.cell_type;
    }

    pub fn to_string(&self, position: VectorU16) -> String {
        format!(
            "{}{}{}{}",
            Goto(position.x() + 1, position.y() + 1),
            Fg(self.color),
            self.char,
            Fg(Reset)
        )
    }
}
