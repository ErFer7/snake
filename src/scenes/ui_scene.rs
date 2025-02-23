use std::collections::HashMap;

use crate::{
    cells::{cell_matrix::CellMatrix, color::Color, vector::Vector},
    core::{events::Event, gameplay_context::GameplayContext, terminal::Terminal},
    ui::{button::Button, selector::Selector, text::Text, ui_element::Alignment},
    VERSION,
};

use super::scene::Scene;

pub struct UiScene {
    name: String,
    cell_matrix: CellMatrix,
    texts: HashMap<String, Text>,
    selector: Selector,
}

impl Scene for UiScene {
    fn new(name: String, width: u16, height: u16) -> Self {
        UiScene {
            name,
            cell_matrix: CellMatrix::new(width, height),
            texts: HashMap::new(),
            selector: Selector::new(),
        }
    }

    fn name(&self) -> String {
        return self.name.clone();
    }

    fn add_text(&mut self, text: Text) {
        self.texts.insert(text.name(), text);
    }

    fn update(
        &mut self,
        terminal: &mut Terminal,
        gameplay_context: GameplayContext,
        _: f64,
    ) -> (Event, GameplayContext) {
        let pressed_key = terminal.get_pressed_key();
        let event = self.selector.update(pressed_key, &mut self.cell_matrix);

        return (event, gameplay_context);
    }

    fn render_texts(&mut self) {
        for (_, text) in self.texts.iter_mut() {
            text.render(&mut self.cell_matrix);
        }
    }

    fn render(&mut self) {
        self.cell_matrix.clear();
        self.render_texts();
        self.selector.render(&mut self.cell_matrix);
    }

    fn write(&mut self, terminal: &mut Terminal) {
        self.cell_matrix.write(terminal);
    }
}

impl UiScene {
    pub fn selector_mut(&mut self) -> &mut Selector {
        &mut self.selector
    }
}

pub fn build_main_menu_scene(width: u16, height: u16) -> UiScene {
    let mut ui_scene = UiScene::new("main_menu".to_string(), width, height);

    let top_divider = Text::new(
        "top_divider".to_string(),
        Vector::<i32>::new(0, 0),
        Alignment::Top,
        "━".repeat(width as usize),
        width,
        height,
        Color::LightGreen.to_rgb(),
    );

    let bottom_divider = Text::new(
        "bottom_divider".to_string(),
        Vector::<i32>::new(0, 0),
        Alignment::Bottom,
        "━".repeat(width as usize),
        width,
        height,
        Color::LightGreen.to_rgb(),
    );

    let back_image = Text::new(
        "back_image".to_string(),
        Vector::<i32>::new(0, 10),
        Alignment::Center,
        [
            "⠀⠀⠀⠀⠀⠀⠀⢀⣠⣤⣶⣶⣿⣿⣿⣿⣿⣷⣶⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣶⣶⡿⠿⢿⣿⣶⣶⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⣠⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠞⠋⠉⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⣿⣷⣄⠀⠀⠀⠀⠀",
            "⠀⠀⠀⣠⣾⣿⣿⣿⣿⠿⠛⠉⠁⠀⠀⠀⠀⠉⠙⠻⢿⣿⣿⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀⠀⣀⣴⣶⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⣷⣄⠀⠀⠀",
            "⠀⠀⣼⣿⣿⣿⡿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣷⡀⠀⠀⠀⢀⣶⣿⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⣧⠀⠀",
            "⠀⣼⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣿⣿⣿⣄⠀⠀⣿⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⣧⠀",
            "⢸⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣿⣿⣿⢂⣾⣿⣿⣿⠿⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⡄",
            "⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡿⢡⣿⣿⣿⡿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⡇",
            "⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣱⣿⣿⣿⡿⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⡇",
            "⢿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⣿⡟⣴⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⣿⡇",
            "⠸⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⠏⢸⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿⠁",
            "⠀⢻⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⡿⠃⠀⠀⠹⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣿⣿⠃⠀",
            "⠀⠀⠹⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⣿⠟⠁⠀⠀⠀⠀⠈⢻⣿⣿⣿⣷⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⡿⠃⠀⠀",
            "⠀⠀⠀⠈⠻⣿⣿⣿⣿⣶⣤⣀⣀⠀⠀⠀⣀⣀⣤⣶⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⣶⣤⣀⣀⠀⠀⠀⢀⣀⣤⣶⣿⣿⣿⣿⠟⠁⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠈⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠛⠁⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠻⠿⠿⠿⠿⠿⠟⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠻⠿⢿⣿⣿⣿⠿⠿⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀",
        ]
        .join("\n"),
        width,
        height,
        Color::LightGreen.to_rgb(),
    );

    let title = Text::new(
        "title".to_string(),
        Vector::<i32>::new(0, 5),
        Alignment::Top,
        [
            "███████╗███╗   ██╗ █████╗ ██╗  ██╗███████╗",
            "██╔════╝████╗  ██║██╔══██╗██║ ██╔╝██╔════╝",
            "███████╗██╔██╗ ██║███████║█████╔╝ █████╗  ",
            "╚════██║██║╚██╗██║██╔══██║██╔═██╗ ██╔══╝  ",
            "███████║██║ ╚████║██║  ██║██║  ██╗███████╗",
            "╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝",
        ]
        .join("\n"),
        width,
        height,
        Color::LightGreen.to_rgb(),
    );

    let start = Button::new(
        "start".to_string(),
        Vector::<i32>::new(0, -5),
        Alignment::Center,
        "START".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Start,
    );

    let exit = Button::new(
        "exit".to_string(),
        Vector::<i32>::new(0, -3),
        Alignment::Center,
        "EXIT".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Exit,
    );

    let version = Text::new(
        "version".to_string(),
        Vector::<i32>::new(0, -3),
        Alignment::Bottom,
        VERSION.to_string(),
        width,
        height,
        Color::LightCyan.to_rgb(),
    );

    let info = Text::new(
        "info".to_string(),
        Vector::<i32>::new(0, -2),
        Alignment::Bottom,
        "Written in Rust by ErFer7".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    ui_scene.add_text(top_divider);
    ui_scene.add_text(bottom_divider);
    ui_scene.add_text(back_image);
    ui_scene.add_text(title);
    ui_scene.add_text(version);
    ui_scene.add_text(info);

    let selector = ui_scene.selector_mut();

    selector.add_button(start);
    selector.add_button(exit);

    ui_scene.render();

    return ui_scene;
}

pub fn build_paused_scene(width: u16, height: u16) -> UiScene {
    let mut ui_scene = UiScene::new("paused".to_string(), width, height);

    let top_divider = Text::new(
        "top_divider".to_string(),
        Vector::<i32>::new(0, 0),
        Alignment::Top,
        "━".repeat(width as usize),
        width,
        height,
        Color::White.to_rgb(),
    );

    let bottom_divider = Text::new(
        "bottom_divider".to_string(),
        Vector::<i32>::new(0, 0),
        Alignment::Bottom,
        "━".repeat(width as usize),
        width,
        height,
        Color::White.to_rgb(),
    );

    let title = Text::new(
        "title".to_string(),
        Vector::<i32>::new(0, 5),
        Alignment::Top,
        [
            "██████╗  █████╗ ██╗   ██╗███████╗███████╗██████╗ ",
            "██╔══██╗██╔══██╗██║   ██║██╔════╝██╔════╝██╔══██╗",
            "██████╔╝███████║██║   ██║███████╗█████╗  ██║  ██║",
            "██╔═══╝ ██╔══██║██║   ██║╚════██║██╔══╝  ██║  ██║",
            "██║     ██║  ██║╚██████╔╝███████║███████╗██████╔╝",
            "╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝╚═════╝",
        ]
        .join("\n"),
        width,
        height,
        Color::White.to_rgb(),
    );

    let resume = Button::new(
        "resume".to_string(),
        Vector::<i32>::new(0, -5),
        Alignment::Center,
        "RESUME".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Resume,
    );

    let restart = Button::new(
        "restart".to_string(),
        Vector::<i32>::new(0, -3),
        Alignment::Center,
        "RESTART".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Restart,
    );

    let exit = Button::new(
        "exit".to_string(),
        Vector::<i32>::new(0, -1),
        Alignment::Center,
        "END".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::End,
    );

    ui_scene.add_text(top_divider);
    ui_scene.add_text(bottom_divider);
    ui_scene.add_text(title);

    let selector = ui_scene.selector_mut();

    selector.add_button(resume);
    selector.add_button(restart);
    selector.add_button(exit);

    ui_scene.render();

    return ui_scene;
}

pub fn build_game_over_scene(width: u16, height: u16) -> UiScene {
    let mut ui_scene = UiScene::new("game_over".to_string(), width, height);

    let top_divider = Text::new(
        "top_divider".to_string(),
        Vector::<i32>::new(0, 0),
        Alignment::Top,
        "━".repeat(width as usize),
        width,
        height,
        Color::LightRed.to_rgb(),
    );

    let bottom_divider = Text::new(
        "bottom_divider".to_string(),
        Vector::<i32>::new(0, 0),
        Alignment::Bottom,
        "━".repeat(width as usize),
        width,
        height,
        Color::LightRed.to_rgb(),
    );

    let title = Text::new(
        "title".to_string(),
        Vector::<i32>::new(0, 5),
        Alignment::Top,
        [
            " ██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ██╗   ██╗███████╗██████╗ ",
            "██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██║   ██║██╔════╝██╔══██╗",
            "██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║██║   ██║█████╗  ██████╔╝",
            "██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗",
            "╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║",
            " ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝",
        ]
        .join("\n"),
        width,
        height,
        Color::LightRed.to_rgb(),
    );

    let restart = Button::new(
        "restart".to_string(),
        Vector::<i32>::new(0, -3),
        Alignment::Center,
        "RESTART".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightRed.to_rgb(),
        Event::Restart,
    );

    let menu = Button::new(
        "menu".to_string(),
        Vector::<i32>::new(0, -1),
        Alignment::Center,
        "MENU".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightRed.to_rgb(),
        Event::GoToMenu,
    );

    ui_scene.add_text(top_divider);
    ui_scene.add_text(bottom_divider);
    ui_scene.add_text(title);

    let selector = ui_scene.selector_mut();

    selector.add_button(restart);
    selector.add_button(menu);

    ui_scene.render();

    return ui_scene;
}
