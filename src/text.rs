use termion::color::Rgb;

use crate::cell::Cell;
use crate::cell_matrix::CellMatrix;

#[derive(Clone)]
pub enum Alignment {
    TopLeft,
    Top,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    Bottom,
    BottomRight,
}

pub struct Text {
    aligned_x: u16,
    aligned_y: u16,
    width: u16,
    height: u16,
    string: String,
    color: Rgb,
}

impl Text {
    pub fn new(
        x: i32,
        y: i32,
        alignment: Alignment,
        string: String,
        cell_matrix_width: u16,
        cell_matrix_height: u16,
        color: Rgb,
    ) -> Text {
        let size = calculated_string_box_size(&string);
        let width = size.0;
        let height = size.1;
        let aligned_offset = calculated_aligned_offset(
            alignment.clone(),
            width,
            height,
            cell_matrix_width,
            cell_matrix_height,
        );

        return Text {
            aligned_x: (aligned_offset.0 as i32 + x) as u16,
            aligned_y: (aligned_offset.1 as i32 + y) as u16,
            width,
            height,
            string,
            color,
        };
    }

    pub fn set_color(&mut self, color: Rgb) {
        self.color = color;
    }

    pub fn set_string(&mut self, string: String) {
        self.string = string;
    }

    pub fn render(&self, cell_matrix: &mut CellMatrix) {
        let mut string_index = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let character = self.string.chars().nth(string_index);

                if character == Some('\n') {
                    string_index += 1;
                }

                let cell = match self.string.chars().nth(string_index) {
                    Some(ch) => Cell::new_typeless(ch, self.color),
                    None => Cell::new_colorless(' '),
                };

                cell_matrix.set_cell(x + self.aligned_x, y + self.aligned_y, cell);
                string_index += 1;
            }
        }
    }
}

fn calculated_aligned_offset(
    alignment: Alignment,
    width: u16,
    height: u16,
    cell_matrix_width: u16,
    cell_matrix_height: u16,
) -> (u16, u16) {
    let x_offset = match alignment {
        Alignment::TopLeft | Alignment::CenterLeft | Alignment::BottomLeft => 0,
        Alignment::Top | Alignment::Center | Alignment::Bottom => (cell_matrix_width - width) / 2,
        Alignment::TopRight | Alignment::CenterRight | Alignment::BottomRight => {
            cell_matrix_width - width
        }
    };

    let y_offset = match alignment {
        Alignment::TopLeft | Alignment::Top | Alignment::TopRight => 0,
        Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight => {
            (cell_matrix_height - height) / 2
        }
        Alignment::BottomLeft | Alignment::Bottom | Alignment::BottomRight => {
            cell_matrix_height - height
        }
    };

    return (x_offset, y_offset);
}

fn calculated_string_box_size(string: &str) -> (u16, u16) {
    let mut width: usize = 0;
    let mut height: usize = 0;

    for line in string.lines() {
        let line_width = line.chars().count();

        if line_width > width {
            width = line_width;
        }

        height += 1;
    }

    return (width as u16, height as u16);
}
