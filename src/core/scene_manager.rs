use std::collections::HashMap;

use crate::scenes::scene::Scene;

use super::{events::Event, gameplay_context::GameplayContext};

pub struct SceneManager {
    scenes: HashMap<String, Box<dyn Scene>>,
    current_scene: Option<Box<dyn Scene>>,
    exit: bool,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: None,
            exit: false,
        }
    }

    pub fn current_scene_mut(&mut self) -> &mut Option<Box<dyn Scene>> {
        return &mut self.current_scene;
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene>) {
        self.scenes.insert(scene.name(), scene);
    }

    pub fn set_current_scene(&mut self, name: &str) {
        if let Some(scene) = self.scenes.remove(name) {
            if let Some(previous_scene) = self.current_scene.take() {
                let previous_name = previous_scene.name();
                self.scenes.insert(previous_name, previous_scene);
            }

            self.current_scene = Some(scene);
            self.current_scene.as_mut().unwrap().render();
        }
    }

    pub fn exit(&self) -> bool {
        return self.exit;
    }

    pub fn handle_update_result(
        &mut self,
        event: Event,
        gameplay_context: GameplayContext,
    ) -> GameplayContext {
        match event {
            Event::Start => {
                self.set_current_scene("gameplay");
                return GameplayContext::new();
            }
            Event::Pause => {
                self.set_current_scene("paused");
            }
            Event::Resume => {
                self.set_current_scene("gameplay");
            }
            Event::Restart => {
                self.set_current_scene("gameplay");
                return GameplayContext::new();
            }
            Event::End => {
                self.set_current_scene("game_over");
            }
            Event::GoToMenu => {
                self.set_current_scene("main_menu");
            }
            Event::Exit => {
                self.exit = true;
            }
            Event::None => {}
        }

        return gameplay_context;
    }
}
