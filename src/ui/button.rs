use termion::color::Rgb;

use crate::{
    cells::{cell_matrix::CellMatrix, vector::Vector},
    core::events::Event,
    ui::text::Text,
};

use super::ui_element::Orientation;

pub struct Button {
    text: Text,
    bg_color: Rgb,
    fg_color: Rgb,
    selected_bg_color: Rgb,
    selected_fg_color: Rgb,
    event: Event,
}

impl Button {
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
        selected_bg_color: Rgb,
        selected_fg_color: Rgb,
        event: Event,
    ) -> Button {
        let padded_string = format!(" {} ", string);

        let text = Text::new(
            name,
            position,
            anchor,
            alignment,
            padded_string,
            cell_matrix_width,
            cell_matrix_height,
            bg_color,
            fg_color,
        );

        return Button {
            text,
            bg_color,
            fg_color,
            selected_bg_color,
            selected_fg_color,
            event,
        };
    }

    pub fn event(&self) -> Event {
        return self.event.clone();
    }

    pub fn select(&mut self) {
        self.text.set_bg_color(self.selected_bg_color);
        self.text.set_fg_color(self.selected_fg_color);
    }

    pub fn deselect(&mut self) {
        self.text.set_bg_color(self.bg_color);
        self.text.set_fg_color(self.fg_color);
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.text.render(cell_matrix);
    }
}
