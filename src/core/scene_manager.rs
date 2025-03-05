use std::collections::HashMap;

use crate::scenes::{gameplay_scene::GameplayScene, scene::Scene};

use super::events::Event;

pub struct SceneManager {
    scenes: HashMap<String, Box<dyn Scene>>,
    current_scene: Option<Box<dyn Scene>>,
    exit: bool,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        return SceneManager {
            scenes: HashMap::new(),
            current_scene: None,
            exit: false,
        };
    }

    pub fn current_scene_mut(&mut self) -> &mut Option<Box<dyn Scene>> {
        return &mut self.current_scene;
    }

    pub fn set_current_scene(&mut self, name: &str) {
        if let Some(previous_scene) = self.current_scene.take() {
            let previous_name = previous_scene.name();
            self.scenes.insert(previous_name, previous_scene);
        }

        self.current_scene = self.scenes.remove(name);
        self.current_scene.as_mut().unwrap().render();
    }

    pub fn exit(&self) -> bool {
        return self.exit;
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene>) {
        self.scenes.insert(scene.name(), scene);
    }

    pub fn handle_update_result(&mut self, event: Event) {
        match event {
            Event::Start => {
                self.start_new_game();
                self.set_current_scene("gameplay");
            }
            Event::Pause => {
                self.set_current_scene("paused");
            }
            Event::Resume => {
                self.set_current_scene("gameplay");
            }
            Event::Restart => {
                self.start_new_game();
                self.set_current_scene("gameplay");
            }
            Event::End => {
                self.update_game_over_scene();
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
    }

    fn start_new_game(&mut self) {
        self.scenes
            .get_mut("gameplay")
            .unwrap()
            .as_any_mut()
            .downcast_mut::<GameplayScene>()
            .unwrap()
            .start_new_game();
    }

    fn update_game_over_scene(&mut self) {
        let score = if let Some(scene) = self.scenes.get_mut("gameplay") {
            scene
                .as_any_mut()
                .downcast_mut::<GameplayScene>()
                .unwrap()
                .score()
        } else if let Some(scene) = self.current_scene.as_mut() {
            scene
                .as_any_mut()
                .downcast_mut::<GameplayScene>()
                .unwrap()
                .score()
        } else {
            0
        };

        self.scenes
            .get_mut("game_over")
            .unwrap()
            .set_text_string("score", format!("{:010}", score));
    }
}
