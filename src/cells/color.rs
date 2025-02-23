use termion::color;

#[allow(dead_code)]
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