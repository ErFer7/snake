use termion::{
    color::{Bg, Fg, Reset, Rgb},
    cursor::Goto,
};

use super::{color::Color, vector::Vector};

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
    bg_color: Rgb,
    fg_color: Rgb,
    cell_type: CellType,
}

impl Cell {
    pub fn new(char: char, bg_color: Rgb, fg_color: Rgb, cell_type: CellType) -> Cell {
        return Cell {
            char,
            bg_color,
            fg_color,
            cell_type,
        };
    }

    pub fn new_typeless(char: char, bg_color: Rgb, fg_color: Rgb) -> Cell {
        return Cell {
            char,
            bg_color,
            fg_color,
            cell_type: CellType::Solid,
        };
    }

    pub fn new_colorless(char: char) -> Cell {
        return Cell {
            char,
            bg_color: Color::Black.to_rgb(),
            fg_color: Color::White.to_rgb(),
            cell_type: CellType::Solid,
        };
    }

    pub fn new_empty() -> Cell {
        return Cell {
            char: ' ',
            bg_color: Color::Black.to_rgb(),
            fg_color: Color::White.to_rgb(),
            cell_type: CellType::Empty,
        };
    }

    pub fn cell_type(&self) -> CellType {
        return self.cell_type;
    }

    pub fn to_string(&self, position: &Vector<u16>) -> String {
        return format!(
            "{}{}{}{}{}",
            Goto(position.x() + 1, position.y() + 1),
            Bg(self.bg_color),
            Fg(self.fg_color),
            self.char,
            Fg(Reset),
        );
    }
}
