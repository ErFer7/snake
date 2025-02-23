use crate::{
    core::{events::Event, gameplay_context::GameplayContext, terminal::Terminal},
    ui::text::Text,
};

pub trait Scene {
    fn new(name: String, width: u16, height: u16) -> Self
    where
        Self: Sized;
    fn name(&self) -> String;
    fn add_text(&mut self, text: Text);
    fn update(
        &mut self,
        terminal: &mut Terminal,
        gameplay_context: GameplayContext,
        current_fps: f64,
    ) -> (Event, GameplayContext);
    fn render_texts(&mut self);
    fn render(&mut self);
    fn write(&mut self, terminal: &mut Terminal);
}
