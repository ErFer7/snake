use std::collections::HashMap;

use crate::{
    cells::{cell::CellType, cell_matrix::CellMatrix, color::Color, vector::Vector},
    core::{events::Event, gameplay_context::GameplayContext, terminal::Terminal},
    gameplay::{fruit::Fruit, snake::Snake, wall::Wall},
    ui::{text::Text, ui_element::Alignment},
    SNAKE_SPEED,
};

use super::scene::Scene;

pub struct GameplayScene {
    name: String,
    cell_matrix: CellMatrix,
    texts: HashMap<String, Text>,
    gameplay_area_origin: Vector<u16>,
    gameplay_area_extension: Vector<u16>,
    wall: Wall,
    snake: Snake,
    fruit: Fruit,
}

impl Scene for GameplayScene {
    fn new(name: String, width: u16, height: u16) -> Self {
        GameplayScene {
            name,
            cell_matrix: CellMatrix::new(width, height),
            texts: HashMap::new(),
            gameplay_area_origin: Vector::<u16>::new(1, 1),
            gameplay_area_extension: Vector::<u16>::new(width - 1, height - 2),
            wall: Wall::new(width, height - 1),
            snake: Snake::none(),
            fruit: Fruit::none(),
        }
    }

    fn add_text(&mut self, text: Text) {
        self.texts.insert(text.name(), text);
    }

    fn update(
        &mut self,
        terminal: &mut Terminal,
        gameplay_context: GameplayContext,
        current_fps: f64,
    ) -> (Event, GameplayContext) {
        if gameplay_context.start_new_game() {
            return self.start_new_game(gameplay_context);
        }

        self.update_fps_text(current_fps);
        self.snake.render(&mut self.cell_matrix);
        self.fruit.render(&mut self.cell_matrix);

        let pressed_key = terminal.get_pressed_key();

        if pressed_key == Some(termion::event::Key::Esc) {
            return (Event::Pause, gameplay_context);
        }

        self.snake.update(pressed_key);

        let head = self.snake.move_forward();

        return self.handle_head_update(head, gameplay_context);
    }

    fn write(&mut self, terminal: &mut Terminal) {
        self.cell_matrix.write(terminal);
    }

    fn name(&self) -> String {
        return self.name.clone();
    }

    fn render_texts(&mut self) {
        for (_, text) in self.texts.iter_mut() {
            text.render(&mut self.cell_matrix);
        }
    }

    fn render(&mut self) {
        self.cell_matrix.clear();
        self.wall.render(&mut self.cell_matrix);
        self.render_texts();
    }
}

impl GameplayScene {
    fn start_new_game(&mut self, gameplay_context: GameplayContext) -> (Event, GameplayContext) {
        self.snake = Snake::new(
            Vector::<u16>::new(self.cell_matrix.width() / 2, self.cell_matrix.height() / 2),
            SNAKE_SPEED,
        );

        self.fruit = Fruit::new(
            &self.cell_matrix,
            self.gameplay_area_origin.clone(),
            self.gameplay_area_extension.clone(),
        );

        return (
            Event::None,
            GameplayContext::new_game_started(gameplay_context),
        );
    }

    fn update_fps_text(&mut self, current_fps: f64) {
        self.texts
            .get_mut("fps")
            .unwrap()
            .set_string(format!("{:06.2}", current_fps));
        self.render_texts();
    }

    fn update_score_text(&mut self, score: u32) {
        self.texts
            .get_mut("score")
            .unwrap()
            .set_string(format!("{:010}", score + 1));
        self.render_texts();
    }

    fn handle_head_update(
        &mut self,
        head: Option<Vector<u16>>,
        gameplay_context: GameplayContext,
    ) -> (Event, GameplayContext) {
        if head.is_some() {
            if let Some(cell) = self.cell_matrix.get_cell(head.unwrap()) {
                match cell.cell_type() {
                    CellType::Solid | CellType::Snake => {
                        return (Event::End, gameplay_context);
                    }
                    CellType::Fruit => {
                        self.snake.grow();
                        self.update_score_text(gameplay_context.score() + 1);

                        self.fruit = Fruit::new(
                            &self.cell_matrix,
                            self.gameplay_area_origin.clone(),
                            self.gameplay_area_extension.clone(),
                        );

                        return (
                            Event::None,
                            GameplayContext::new_incremented(gameplay_context),
                        );
                    }
                    _ => (),
                }
            }
        }

        return (Event::None, gameplay_context);
    }
}

pub fn build_gameplay_scene(width: u16, height: u16) -> GameplayScene {
    let mut gameplay_scene = GameplayScene::new("gameplay".to_string(), width, height);

    let score_label = Text::new(
        "score_label".to_string(),
        Vector::<i32>::zero(),
        Alignment::BottomLeft,
        "Score: ".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    let score = Text::new(
        "score".to_string(),
        Vector::<i32>::new(7, 0),
        Alignment::BottomLeft,
        "0000000000".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    let fps_label = Text::new(
        "fps_label".to_string(),
        Vector::<i32>::new(18, 0),
        Alignment::BottomLeft,
        "FPS: ".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    let fps = Text::new(
        "fps".to_string(),
        Vector::<i32>::new(23, 0),
        Alignment::BottomLeft,
        "000.00".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    gameplay_scene.add_text(score_label);
    gameplay_scene.add_text(score);
    gameplay_scene.add_text(fps_label);
    gameplay_scene.add_text(fps);

    gameplay_scene.render();

    return gameplay_scene;
}
