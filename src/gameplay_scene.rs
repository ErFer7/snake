use rand::Rng;

use crate::{
    cell::{Cell, CellType, Color},
    cell_matrix::CellMatrix,
    events::Event,
    global_context::GameplayContext,
    scene::{Scene, UpdateResult},
    snake::{Direction, Snake},
    terminal::Terminal,
    text::{Alignment, Text},
    world::generate_walls,
    SNAKE_SPEED,
};

pub struct GameplayScene {
    name: String,
    cell_matrix: CellMatrix,
    texts: Vec<Text>,
    score: u32,
    fruit_position: (u16, u16),
    snake: Snake,
}

impl Scene for GameplayScene {
    fn new(name: String, width: u16, height: u16) -> Self {
        let mut cell_matrix = CellMatrix::new(width, height);
        generate_walls(&mut cell_matrix, width, height);

        GameplayScene {
            name,
            cell_matrix,
            texts: Vec::new(),
            score: 0,
            fruit_position: (0, 0),
            snake: Snake::new(0, 0, 0.0),
        }
    }

    fn add_text(&mut self, text: Text) {
        self.texts.push(text);
        self.texts.last().unwrap().render(&mut self.cell_matrix);
    }

    fn update(
        &mut self,
        terminal: &mut Terminal,
        gameplay_context: GameplayContext,
    ) -> UpdateResult {
        if gameplay_context.start_new_game() {
            self.start_gameplay();

            return UpdateResult::none(GameplayContext::new_game_started(gameplay_context));
        }

        self.snake.render(&mut self.cell_matrix);
        self.render_score_text();

        let pressed_key = terminal.get_pressed_key();

        match pressed_key {
            Some(termion::event::Key::Up) => {
                if self.snake.direction() != Direction::Down {
                    self.snake.change_direction(Direction::Up);
                }
            }
            Some(termion::event::Key::Down) => {
                if self.snake.direction() != Direction::Up {
                    self.snake.change_direction(Direction::Down);
                }
            }
            Some(termion::event::Key::Left) => {
                if self.snake.direction() != Direction::Right {
                    self.snake.change_direction(Direction::Left);
                }
            }
            Some(termion::event::Key::Right) => {
                if self.snake.direction() != Direction::Left {
                    self.snake.change_direction(Direction::Right);
                }
            }
            Some(termion::event::Key::Esc) => {
                return UpdateResult::new(Event::Pause, gameplay_context);
            }
            _ => (),
        }

        let head_position = self.snake.move_forward();

        if let Some((x, y)) = head_position {
            if let Some(cell) = self.cell_matrix.get_cell(x, y) {
                match cell.cell_type() {
                    CellType::Solid | CellType::Snake => {
                        return UpdateResult::new(Event::End, gameplay_context);
                    }
                    CellType::Fruit => {
                        self.snake.grow();
                        self.texts[1].set_string(format!("{:010}", gameplay_context.score() + 1));
                        self.generate_fruit(self.cell_matrix.width(), self.cell_matrix.height());

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

    fn print(&mut self) {
        self.cell_matrix.print();
        self.cell_matrix.clear_type(CellType::Snake);
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}

impl GameplayScene {
    pub fn start_gameplay(&mut self) {
        self.cell_matrix.clear();

        let width = self.cell_matrix.width();
        let height = self.cell_matrix.height();

        generate_walls(&mut self.cell_matrix, width, height - 1);

        self.render_score_text();

        self.snake = Snake::new(
            self.cell_matrix.width() / 2,
            self.cell_matrix.height() / 2,
            SNAKE_SPEED,
        );
        self.score = 0;
        self.generate_fruit(self.cell_matrix.width(), self.cell_matrix.height());
    }

    fn generate_fruit(&mut self, width: u16, height: u16) {
        loop {
            let mut rng = rand::rng();
            let x = rng.random_range(1..width - 1);
            let y = rng.random_range(1..height - 1);

            self.fruit_position = (x, y);

            if let Some(cell) = self
                .cell_matrix
                .get_cell(self.fruit_position.0, self.fruit_position.1)
            {
                if cell.cell_type() == CellType::Empty {
                    self.cell_matrix.set_cell(
                        x,
                        y,
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
        0,
        0,
        Alignment::BottomLeft,
        "Score: ".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    let score = Text::new(
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

    return gameplay_scene;
}
