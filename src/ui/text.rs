use termion::color::Rgb;

use crate::cells::cell_group::CellGroup;
use crate::cells::cell_matrix::CellMatrix;
use crate::cells::{cell::Cell, vector::VectorU16};

use super::ui_element::{Alignment, UiElement};

pub struct Text {
    ui_element: UiElement,
    string: String,
    color: Rgb,
}

impl Text {
    pub fn new(
        name: String,
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
        let mut ui_element = UiElement::new(
            name,
            x,
            y,
            alignment.clone(),
            width,
            height,
            cell_matrix_width,
            cell_matrix_height,
        );
        let aligned_x = ui_element.aligned_x();
        let aligned_y = ui_element.aligned_y();

        update_cell_group(
            &mut ui_element.cell_group_mut(),
            aligned_x,
            aligned_y,
            width,
            height,
            string.clone(),
            color,
        );

        return Text {
            ui_element,
            string,
            color,
        };
    }

    pub fn name(&self) -> String {
        return self.ui_element.name();
    }

    pub fn set_string(&mut self, string: String) {
        self.string = string;

        self.update_cell_group_wrapper();
    }

    pub fn set_color(&mut self, color: Rgb) {
        self.color = color;

        self.update_cell_group_wrapper();
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.ui_element.render(cell_matrix);
    }

    fn update_cell_group_wrapper(&mut self) {
        let aligned_x = self.ui_element.aligned_x();
        let aligned_y = self.ui_element.aligned_y();
        let width = self.ui_element.width();
        let height = self.ui_element.height();

        update_cell_group(
            &mut self.ui_element.cell_group_mut(),
            aligned_x,
            aligned_y,
            width,
            height,
            self.string.clone(),
            self.color,
        );
    }
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

fn update_cell_group(
    cell_group: &mut CellGroup,
    aligned_x: u16,
    aligned_y: u16,
    width: u16,
    height: u16,
    string: String,
    color: Rgb,
) {
    let mut string_index = 0;

    for y in 0..height {
        for x in 0..width {
            // TODO: Handle string resizing
            let character = string.chars().nth(string_index);

            if character == Some('\n') {
                string_index += 1;
            }

            let cell = match string.chars().nth(string_index) {
                Some(ch) => Cell::new_typeless(ch, color),
                None => Cell::new_colorless(' '),
            };

            cell_group.set_cell(VectorU16::new(x + aligned_x, y + aligned_y), cell);
            string_index += 1;
        }
    }
}
