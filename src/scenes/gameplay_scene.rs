use rand::Rng;

use crate::{
    cells::{
        cell::{Cell, CellType},
        cell_matrix::CellMatrix,
        color::Color,
        vector::VectorU16,
    },
    events::Event,
    gameplay::snake::Snake,
    global_context::GameplayContext,
    terminal::Terminal,
    ui::{text::Text, ui_element::Alignment},
    wall::Wall,
    SNAKE_SPEED,
};

use super::scene::{Scene, UpdateResult};

pub struct GameplayScene {
    name: String,
    cell_matrix: CellMatrix,
    texts: Vec<Text>,
    fruit_position: (u16, u16),
    snake: Snake,
    wall: Wall,
}

impl Scene for GameplayScene {
    fn new(name: String, width: u16, height: u16) -> Self {
        GameplayScene {
            name,
            cell_matrix: CellMatrix::new(width, height),
            texts: Vec::new(),
            fruit_position: (0, 0),
            snake: Snake::new(0, 0, 0.0),
            wall: Wall::new(width, height),
        }
    }

    fn add_text(&mut self, text: Text) {
        self.texts.push(text);
    }

    fn update(
        &mut self,
        terminal: &mut Terminal,
        gameplay_context: GameplayContext,
    ) -> UpdateResult {
        if gameplay_context.start_new_game() {
            self.snake = Snake::new(
                self.cell_matrix.width() / 2,
                self.cell_matrix.height() / 2,
                SNAKE_SPEED,
            );
            self.generate_fruit(self.cell_matrix.width(), self.cell_matrix.height());

            return UpdateResult::none(GameplayContext::new_game_started(gameplay_context));
        }

        self.snake.render(&mut self.cell_matrix);

        let pressed_key = terminal.get_pressed_key();

        if pressed_key == Some(termion::event::Key::Esc) {
            return UpdateResult::new(Event::Pause, gameplay_context);
        }

        self.snake.update(pressed_key);

        let head_position = self.snake.move_forward();

        if let Some((x, y)) = head_position {
            if let Some(cell) = self.cell_matrix.get_cell(VectorU16::new(x, y)) {
                match cell.cell_type() {
                    CellType::Solid | CellType::Snake => {
                        return UpdateResult::new(Event::End, gameplay_context);
                    }
                    CellType::Fruit => {
                        self.snake.grow();
                        self.texts[1].set_string(format!("{:010}", gameplay_context.score() + 1));
                        self.generate_fruit(self.cell_matrix.width(), self.cell_matrix.height());
                        self.render_score_text();

                        return UpdateResult::none(GameplayContext::new_incremented(
                            gameplay_context,
                        ));
                    }
                    _ => (),
                }
            }
        }

        return UpdateResult::none(gameplay_context);
    }

    fn write(&mut self, terminal: &mut Terminal) {
        self.cell_matrix.write(terminal);
    }

    fn name(&self) -> String {
        return self.name.clone();
    }

    fn render_texts(&mut self) {
        for text in self.texts.iter_mut() {
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
    fn generate_fruit(&mut self, width: u16, height: u16) {
        loop {
            let mut rng = rand::rng();
            let x = rng.random_range(1..width - 1);
            let y = rng.random_range(1..height - 1);

            self.fruit_position = (x, y);

            if let Some(cell) = self
                .cell_matrix
                .get_cell(VectorU16::new(self.fruit_position.0, self.fruit_position.1))
            {
                if cell.cell_type() == CellType::Empty {
                    self.cell_matrix.set_cell(
                        VectorU16::new(x, y),
                        Cell::new('█', Color::Red.to_rgb(), CellType::Fruit),
                    );
                    break;
                }
            }
        }
    }

    pub fn render_score_text(&mut self) {
        for text in self.texts.iter_mut() {
            text.render(&mut self.cell_matrix);
        }
    }
}

pub fn build_gameplay_scene(width: u16, height: u16) -> GameplayScene {
    let mut gameplay_scene = GameplayScene::new("gameplay".to_string(), width, height);

    let score_label = Text::new(
        "score_label".to_string(),
        0,
        0,
        Alignment::BottomLeft,
        "Score: ".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    let score = Text::new(
        "score".to_string(),
        7,
        0,
        Alignment::BottomLeft,
        "0000000000".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    gameplay_scene.add_text(score_label);
    gameplay_scene.add_text(score);

    gameplay_scene.render();

    return gameplay_scene;
}
