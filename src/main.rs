mod button;
mod cell;
mod cell_matrix;
mod chronometer;
mod events;
mod gameplay_scene;
mod global_context;
mod scene;
mod snake;
mod terminal;
mod text;
mod ui_scene;
mod world;

use chronometer::Chronometer;
use global_context::GlobalContext;
use terminal::Terminal;

const MINIMUM_WIDTH: u16 = 80;
const MINIMUM_HEIGHT: u16 = 35;
const FPS: u32 = 75;
const SNAKE_SPEED: f32 = 25.0;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut global_context = GlobalContext::new();
    let mut chronometer = Chronometer::new(FPS);

    let mut terminal = Terminal::new();
    let width = terminal.width();
    let height = terminal.height();

    if width < MINIMUM_WIDTH || height < MINIMUM_HEIGHT {
        panic!(
            "The terminal size is too small! Minimum size is {}x{}",
            MINIMUM_WIDTH, MINIMUM_HEIGHT
        );
    }

    global_context.add_scene(Box::new(ui_scene::build_main_menu_scene(width, height)));
    global_context.add_scene(Box::new(gameplay_scene::build_gameplay_scene(
        width, height,
    )));
    global_context.add_scene(Box::new(ui_scene::build_paused_scene(width, height)));
    global_context.add_scene(Box::new(ui_scene::build_game_over_scene(width, height)));

    global_context.set_current_scene("main_menu");

    terminal.hide_cursor();

    loop {
        if !chronometer.update() {
            continue;
        }

        let gameplay_context = global_context.gameplay_context();
        let current_scene = global_context.current_scene_mut().as_mut().unwrap();

        current_scene.print();

        let update_result = current_scene.update(&mut terminal, gameplay_context);

        global_context.handle_update_result(update_result);

        if global_context.exit() {
            break;
        }
    }

    terminal.show_cursor();
    terminal.clear();
}
