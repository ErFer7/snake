use termion::color::Rgb;

use crate::{cell_matrix::CellMatrix, events::Event, text::{Alignment, Text}};

pub struct Button {
    text: Text,
    selected: bool,
    color: Rgb,
    selected_color: Rgb,
    event: Event,
}

impl Button {
    pub fn new(
        x: i32,
        y: i32,
        alignment: Alignment,
        string: String,
        cell_matrix_width: u16,
        cell_matrix_height: u16,
        color: Rgb,
        selected_color: Rgb,
        event: Event,
    ) -> Button {
        let text = Text::new(
            x,
            y,
            alignment,
            string,
            cell_matrix_width,
            cell_matrix_height,
            color,
        );

        return Button {
            text,
            selected: false,
            color,
            selected_color,
            event,
        };
    }

    pub fn select(&mut self) {
        self.selected = true;
        self.text.set_color(self.selected_color);
    }

    pub fn deselect(&mut self) {
        self.selected = false;
        self.text.set_color(self.color);   
    }

    pub fn render(&self, cell_matrix: &mut CellMatrix) {
        self.text.render(cell_matrix);
    }

    pub fn event(&self) -> Event {
        return self.event.clone();
    }
}
