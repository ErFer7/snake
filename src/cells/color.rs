use termion::color::Rgb;

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
    pub fn to_rgb(&self) -> Rgb {
        return match self {
            Color::Black => Rgb(0, 0, 0),
            Color::Red => Rgb(255, 0, 0),
            Color::Green => Rgb(0, 255, 0),
            Color::Yellow => Rgb(255, 255, 0),
            Color::Blue => Rgb(0, 0, 255),
            Color::Magenta => Rgb(255, 0, 255),
            Color::Cyan => Rgb(0, 255, 255),
            Color::White => Rgb(255, 255, 255),
            Color::LightBlack => Rgb(128, 128, 128),
            Color::LightRed => Rgb(255, 128, 128),
            Color::LightGreen => Rgb(128, 255, 128),
            Color::LightYellow => Rgb(255, 255, 128),
            Color::LightBlue => Rgb(128, 128, 255),
            Color::LightMagenta => Rgb(255, 128, 255),
            Color::LightCyan => Rgb(128, 255, 255),
            Color::LightWhite => Rgb(255, 255, 255),
        };
    }
}
