use crate::{
    events::Event,
    global_context::GameplayContext,
    terminal::Terminal,
    ui::{button::Button, text::Text},
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
    ) -> UpdateResult;
    fn render_texts(&mut self);
    fn render(&mut self);
    fn write(&mut self, terminal: &mut Terminal);
}

pub struct UpdateResult {
    event: Event,
    gameplay_context: GameplayContext,
}

impl UpdateResult {
    pub fn new(event: Event, gameplay_context: GameplayContext) -> Self {
        Self {
            event,
            gameplay_context,
        }
    }

    pub fn none(gameplay_context: GameplayContext) -> Self {
        Self {
            event: Event::None,
            gameplay_context,
        }
    }

    pub fn event(&self) -> Event {
        self.event.clone()
    }

    pub fn gameplay_context(&self) -> GameplayContext {
        self.gameplay_context.clone()
    }
}
