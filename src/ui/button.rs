use termion::color::Rgb;

use crate::{cells::{cell_matrix::CellMatrix, vector::Vector}, core::events::Event, ui::text::Text};

use super::ui_element::Alignment;

pub struct Button {
    text: Text,
    color: Rgb,
    selected_color: Rgb,
    event: Event,
}

impl Button {
    pub fn new(
        name: String,
        position: Vector<i32>,
        alignment: Alignment,
        string: String,
        cell_matrix_width: u16,
        cell_matrix_height: u16,
        color: Rgb,
        selected_color: Rgb,
        event: Event,
    ) -> Button {
        let text = Text::new(
            name,
            position,
            alignment,
            string,
            cell_matrix_width,
            cell_matrix_height,
            color,
        );

        return Button {
            text,
            color,
            selected_color,
            event,
        };
    }

    pub fn event(&self) -> Event {
        return self.event.clone();
    }

    pub fn select(&mut self) {
        self.text.set_color(self.selected_color);
    }

    pub fn deselect(&mut self) {
        self.text.set_color(self.color);
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.text.render(cell_matrix);
    }
}
