mod button;
mod cell;
mod cell_matrix;
mod chronometer;
mod gameplay_scene;
mod scene;
mod snake;
mod state_machine;
mod terminal;
mod text;
mod ui_scene;
mod world;

use chronometer::Chronometer;
use scene::Scene;
use state_machine::{Event, State};
use terminal::Terminal;

const FPS: u32 = 75;
const SNAKE_SPEED: f32 = 25.0;

fn main() {
    let mut chronometer = Chronometer::new(FPS);

    let mut terminal = Terminal::new();
    let width = terminal.width();
    let height = terminal.height();

    let mut state = State::MainMenu;
    let mut main_menu = ui_scene::build_main_menu_scene(width, height);
    let mut gameplay = gameplay_scene::build_gameplay_scene(width, height);
    let mut paused = ui_scene::build_paused_scene(width, height);
    let mut game_over = ui_scene::build_game_over_scene(width, height);

    terminal.hide_cursor();

    loop {
        if !chronometer.update() {
            continue;
        }

        match state {
            State::MainMenu => {
                main_menu.print();
                main_menu.update(&mut terminal, &mut state);
            }
            State::Starting => {
                gameplay.start_gameplay();
                state = state.transition(Event::None);
            }
            State::Gameplay => {
                gameplay.print();
                gameplay.update(&mut terminal, &mut state);
            }
            State::Paused => {
                paused.print();
                paused.update(&mut terminal, &mut state);
            }
            State::GameOver => {
                game_over.print();
                game_over.update(&mut terminal, &mut state);
            }
            State::Exit => {
                break;
            }
            State::Invalid => panic!("Invalid state!"),
        }
    }

    terminal.show_cursor();
    terminal.clear();
}
