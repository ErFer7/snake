use crate::{
    button::Button,
    events::Event,
    global_context::GameplayContext,
    terminal::Terminal,
    text::Text,
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
    fn print(&mut self);
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

pub trait ButtonInteractableScene {
    fn add_button(&mut self, button: Button);
    fn render_buttons(&mut self);
}
