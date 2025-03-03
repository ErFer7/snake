mod cells;
mod core;
mod gameplay;
mod scenes;
mod ui;

use core::{
    chronometer::Chronometer, scene_manager::SceneManager,
    terminal::Terminal,
};

use scenes::{
    gameplay_scene::build_gameplay_scene,
    ui_scene::{build_game_over_scene, build_main_menu_scene, build_paused_scene},
};

const MINIMUM_WIDTH: u16 = 80;
const MINIMUM_HEIGHT: u16 = 45;
const FPS: u32 = 256;
const SNAKE_SPEED: f32 = 10.0;
const INITIAL_SNAKE_LENGTH: u16 = 2;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut scene_manager = SceneManager::new();
    let mut chronometer = Chronometer::new(FPS);
    let mut terminal = Terminal::new();

    let width = terminal.width();
    let height = terminal.height();

    scene_manager.add_scene(Box::new(build_main_menu_scene(width, height)));
    scene_manager.add_scene(Box::new(build_gameplay_scene(width, height)));
    scene_manager.add_scene(Box::new(build_paused_scene(width, height)));
    scene_manager.add_scene(Box::new(build_game_over_scene(width, height)));

    scene_manager.set_current_scene("main_menu");

    terminal.hide_cursor();

    loop {
        if !chronometer.update() {
            continue;
        }

        let current_scene = scene_manager.current_scene_mut().as_mut().unwrap();

        current_scene.write(&mut terminal);
        terminal.flush();

        let event = current_scene.update(
            terminal.get_pressed_key(),
            chronometer.current_fps(),
            chronometer.real_frame_duration(),
        );

        scene_manager.handle_update_result(event);

        if scene_manager.exit() {
            break;
        }
    }

    terminal.show_cursor();
    terminal.clear();
}
