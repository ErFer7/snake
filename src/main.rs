mod cells;
mod core;
mod gameplay;
mod scenes;
mod ui;

use core::{
    chronometer::Chronometer, gameplay_context::GameplayContext, scene_manager::SceneManager,
    terminal::Terminal,
};

use scenes::{gameplay_scene, ui_scene};

const MINIMUM_WIDTH: u16 = 80;
const MINIMUM_HEIGHT: u16 = 45;
const FPS: u32 = 75;
const SNAKE_SPEED: f32 = 10.0;
const INITIAL_SNAKE_LENGTH: u16 = 2;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut scene_manager = SceneManager::new();
    let mut chronometer = Chronometer::new(FPS);
    let mut gameplay_context = GameplayContext::new();

    let mut terminal = Terminal::new();
    let width = terminal.width();
    let height = terminal.height();

    if width < MINIMUM_WIDTH || height < MINIMUM_HEIGHT {
        panic!(
            "The terminal size is too small! Minimum size is {}x{}",
            MINIMUM_WIDTH, MINIMUM_HEIGHT
        );
    }

    scene_manager.add_scene(Box::new(ui_scene::build_main_menu_scene(width, height)));
    scene_manager.add_scene(Box::new(gameplay_scene::build_gameplay_scene(
        width, height,
    )));
    scene_manager.add_scene(Box::new(ui_scene::build_paused_scene(width, height)));
    scene_manager.add_scene(Box::new(ui_scene::build_game_over_scene(width, height)));

    scene_manager.set_current_scene("main_menu");

    terminal.hide_cursor();

    loop {
        if !chronometer.update() {
            continue;
        }

        let current_scene = scene_manager.current_scene_mut().as_mut().unwrap();

        current_scene.write(&mut terminal);
        terminal.flush();

        let (event, updated_gameplay_context) =
            current_scene.update(&mut terminal, gameplay_context, chronometer.current_fps());
        gameplay_context = scene_manager.handle_update_result(event, updated_gameplay_context);

        if scene_manager.exit() {
            break;
        }
    }

    terminal.show_cursor();
    terminal.clear();
}
