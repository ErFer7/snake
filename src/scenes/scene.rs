use std::any::Any;

use termion::event::Key;

use crate::{
    core::{events::Event, terminal::Terminal},
    ui::text::Text,
};

pub trait Scene {
    fn new(name: String, width: u16, height: u16) -> Self
    where
        Self: Sized;
    fn name(&self) -> String;
    fn add_text(&mut self, text: Text);
    fn set_text(&mut self, text_name: &str, new_text: String);
    fn update(&mut self, pressed_key: Option<Key>, current_fps: f64, frame_duration: f64) -> Event;
    fn render_texts(&mut self);
    fn render(&mut self);
    fn write(&mut self, terminal: &mut Terminal);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
