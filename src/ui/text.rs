use termion::color::Rgb;

use crate::cells::cell_group::CellGroup;
use crate::cells::cell_matrix::CellMatrix;
use crate::cells::{cell::Cell, vector::Vector};

use super::ui_element::{Orientation, UiElement};

pub struct Text {
    ui_element: UiElement,
    string: String,
    bg_color: Rgb,
    fg_color: Rgb,
}

impl Text {
    pub fn new(
        name: String,
        position: Vector<i32>,
        anchor: Orientation,
        alignment: Orientation,
        string: String,
        cell_matrix_width: u16,
        cell_matrix_height: u16,
        bg_color: Rgb,
        fg_color: Rgb,
    ) -> Text {
        let size = calculated_string_box_size(&string);
        let width = size.0;
        let height: u16 = size.1;
        let mut ui_element = UiElement::new(
            name,
            position,
            anchor,
            alignment,
            width,
            height,
            cell_matrix_width,
            cell_matrix_height,
        );
        let aligned_position = ui_element.aligned_position();

        update_cell_group(
            &mut ui_element.cell_group_mut(),
            aligned_position,
            width,
            height,
            string.clone(),
            bg_color,
            fg_color,
        );

        return Text {
            ui_element,
            string,
            bg_color,
            fg_color,
        };
    }

    pub fn name(&self) -> String {
        return self.ui_element.name();
    }

    pub fn set_string(&mut self, string: String) {
        let resize = self.string.len() == string.len();

        self.string = string;

        if resize {
            let size = calculated_string_box_size(&self.string);

            self.ui_element.set_width(size.0);
            self.ui_element.set_height(size.1);
        }

        self.update_cell_group_wrapper();
    }

    pub fn set_bg_color(&mut self, color: Rgb) {
        self.bg_color = color;

        self.update_cell_group_wrapper();
    }

    pub fn set_fg_color(&mut self, color: Rgb) {
        self.fg_color = color;

        self.update_cell_group_wrapper();
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.ui_element.render(cell_matrix);
    }

    fn update_cell_group_wrapper(&mut self) {
        let aligned_position = self.ui_element.aligned_position();
        let width = self.ui_element.width();
        let height = self.ui_element.height();

        update_cell_group(
            &mut self.ui_element.cell_group_mut(),
            aligned_position,
            width,
            height,
            self.string.clone(),
            self.bg_color,
            self.fg_color,
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
    aligned_position: Vector<u16>,
    width: u16,
    height: u16,
    string: String,
    bg_color: Rgb,
    fg_color: Rgb,
) {
    let mut string_index = 0;

    for y in 0..height {
        for x in 0..width {
            let character = string.chars().nth(string_index);

            if character == Some('\n') {
                string_index += 1;
            }

            let cell = match string.chars().nth(string_index) {
                Some(ch) => Cell::new_typeless(ch, bg_color, fg_color),
                None => Cell::new_colorless(' '),
            };

            cell_group.set_cell(
                Vector::<u16>::new(x + aligned_position.x(), y + aligned_position.y()),
                cell,
            );
            string_index += 1;
        }
    }
}
