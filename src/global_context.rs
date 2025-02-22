use std::collections::HashMap;

use crate::{
    events::Event, scenes::scene::{Scene, UpdateResult},
};

pub struct GlobalContext {
    scenes: HashMap<String, Box<dyn Scene>>,
    current_scene: Option<Box<dyn Scene>>,
    gameplay_context: GameplayContext,
    exit: bool,
}

pub struct GameplayContext {
    score: u32,
    start_new_game: bool,
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: None,
            gameplay_context: GameplayContext {
                score: 0,
                start_new_game: true,
            },
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

    pub fn gameplay_context(&self) -> GameplayContext {
        self.gameplay_context.clone()
    }

    pub fn handle_update_result(&mut self, update_result: UpdateResult) {
        self.gameplay_context = update_result.gameplay_context();

        match update_result.event() {
            Event::Start => {
                self.gameplay_context.reset();
                self.set_current_scene("gameplay");
            }
            Event::Pause => {
                self.set_current_scene("paused");
            }
            Event::Resume => {
                self.set_current_scene("gameplay");
            }
            Event::Restart => {
                self.gameplay_context.reset();
                self.set_current_scene("gameplay");
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
    }
}

impl GameplayContext {
    pub fn reset(&mut self) {
        self.score = 0;
        self.start_new_game = true;
    }

    pub fn new_incremented(gameplay_context: GameplayContext) -> Self {
        Self {
            score: gameplay_context.score + 1,
            start_new_game: gameplay_context.start_new_game,
        }
    }

    pub fn new_game_started(gameplay_context: GameplayContext) -> Self {
        Self {
            score: gameplay_context.score,
            start_new_game: false,
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn start_new_game(&self) -> bool {
        self.start_new_game
    }

    pub(crate) fn clone(&self) -> GameplayContext {
        GameplayContext {
            score: self.score,
            start_new_game: self.start_new_game,
        }
    }
}
