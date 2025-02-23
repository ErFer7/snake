#[derive(Clone)]
pub struct GameplayContext {
    score: u32,
    start_new_game: bool,
}

impl GameplayContext {
    pub fn new() -> Self {
        Self {
            score: 0,
            start_new_game: true,
        }
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
}
