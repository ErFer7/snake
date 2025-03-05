use termion::event::Key;

use crate::{cells::cell_matrix::CellMatrix, core::events::Event};

use super::button::Button;

pub struct Selector {
    buttons: Vec<Button>,
    selected_index: usize,
}

impl Selector {
    pub fn new() -> Selector {
        return Selector {
            buttons: Vec::new(),
            selected_index: 0,
        };
    }

    pub fn add_button(&mut self, button: Button) {
        self.buttons.push(button);

        if self.buttons.len() == 1 {
            self.buttons[0].select();
        }
    }

    pub fn update(&mut self, pressed_key: Option<Key>, cell_matrix: &mut CellMatrix) -> Event {
        match pressed_key {
            Some(Key::Up) => {
                if self.selected_index > 0 {
                    self.update_selection(self.selected_index - 1, cell_matrix);
                }
            }
            Some(Key::Down) => {
                if self.selected_index < self.buttons.len() - 1 {
                    self.update_selection(self.selected_index + 1, cell_matrix);
                }
            }
            Some(Key::Char('\n')) => {
                return self.buttons[self.selected_index].event();
            }
            _ => {}
        }

        return Event::None;
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        for button in self.buttons.iter_mut() {
            button.render(cell_matrix);
        }
    }

    fn update_selection(&mut self, index: usize, cell_matrix: &mut CellMatrix) {
        self.buttons[self.selected_index].deselect();

        self.selected_index = index;

        self.buttons[self.selected_index].select();

        self.render(cell_matrix);
    }
}
