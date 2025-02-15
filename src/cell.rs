use termion::{
    color::{self, Fg, Reset, Rgb},
    cursor::Goto,
};

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
}

impl Color {
    pub fn to_rgb(&self) -> color::Rgb {
        match self {
            Color::Black => color::Rgb(0, 0, 0),
            Color::Red => color::Rgb(255, 0, 0),
            Color::Green => color::Rgb(0, 255, 0),
            Color::Yellow => color::Rgb(255, 255, 0),
            Color::Blue => color::Rgb(0, 0, 255),
            Color::Magenta => color::Rgb(255, 0, 255),
            Color::Cyan => color::Rgb(0, 255, 255),
            Color::White => color::Rgb(255, 255, 255),
            Color::LightBlack => color::Rgb(128, 128, 128),
            Color::LightRed => color::Rgb(255, 128, 128),
            Color::LightGreen => color::Rgb(128, 255, 128),
            Color::LightYellow => color::Rgb(255, 255, 128),
            Color::LightBlue => color::Rgb(128, 128, 255),
            Color::LightMagenta => color::Rgb(255, 128, 255),
            Color::LightCyan => color::Rgb(128, 255, 255),
            Color::LightWhite => color::Rgb(255, 255, 255),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Solid,
    Fruit,
    Snake,
    Empty,
}

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

    pub fn print(&self, x: u16, y: u16) {
        print!(
            "{}{}{}{}",
            Goto(x + 1, y + 1),
            Fg(self.color),
            self.char,
            Fg(Reset)
        );
    }
}
