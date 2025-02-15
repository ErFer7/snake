use crate::{
    button::Button,
    state_machine::State,
    terminal::Terminal,
    text::Text,
};

pub trait Scene {
    fn new(width: u16, height: u16) -> Self where Self: Sized;
    fn add_text(&mut self, text: Text);
    fn update(&mut self, terminal: &mut Terminal, state: &mut State);
    fn print(&mut self);
}

pub trait ButtonInteractableScene {
    fn add_button(&mut self, button: Button);
    fn render_buttons(&mut self);
}
