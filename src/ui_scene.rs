use crate::{
    button::Button,
    cell::Color,
    cell_matrix::CellMatrix,
    scene::{ButtonInteractableScene, Scene},
    state_machine::{Event, State},
    terminal::Terminal,
    text::{Alignment, Text},
    world::generate_walls,
};

pub struct UiScene {
    cell_matrix: CellMatrix,
    texts: Vec<Text>,
    buttons: Vec<Button>,
    selected_button: usize,
}

impl Scene for UiScene {
    fn new(width: u16, height: u16) -> Self {
        let mut ui_cell_matrix = CellMatrix::new(width, height);
        generate_walls(&mut ui_cell_matrix, width, height);

        UiScene {
            cell_matrix: ui_cell_matrix,
            texts: Vec::new(),
            buttons: Vec::new(),
            selected_button: 0,
        }
    }

    fn add_text(&mut self, text: Text) {
        self.texts.push(text);
        self.texts.last().unwrap().render(&mut self.cell_matrix);
    }

    fn update(&mut self, terminal: &mut Terminal, state: &mut State) {
        let pressed_key = terminal.get_pressed_key();

        match pressed_key {
            Some(termion::event::Key::Up) => {
                if self.selected_button > 0 {
                    self.buttons[self.selected_button].deselect();
                    self.selected_button -= 1;
                    self.buttons[self.selected_button].select();
                    self.render_buttons();
                }
            }
            Some(termion::event::Key::Down) => {
                if self.selected_button < self.buttons.len() - 1 {
                    self.buttons[self.selected_button].deselect();
                    self.selected_button += 1;
                    self.buttons[self.selected_button].select();
                    self.render_buttons();
                }
            }
            Some(termion::event::Key::Char('\n')) => {
                let event = self.buttons[self.selected_button].event();
                self.selected_button = 0;
                *state = state.transition(event);
            }
            Some(termion::event::Key::Esc) => {
                *state = state.transition(Event::Exit);
            }
            _ => (),
        }
    }

    fn print(&mut self) {
        self.cell_matrix.print();
    }
}

impl ButtonInteractableScene for UiScene {
    fn add_button(&mut self, mut button: Button) {
        if self.buttons.len() == self.selected_button {
            button.select();
        }
        button.render(&mut self.cell_matrix);
        self.buttons.push(button);
    }

    fn render_buttons(&mut self) {
        for button in self.buttons.iter_mut() {
            button.render(&mut self.cell_matrix);
        }
    }
}

pub fn build_main_menu_scene(width: u16, height: u16) -> UiScene {
    let mut ui_scene = UiScene::new(width, height);

    let title = Text::new(
        0,
        5,
        Alignment::Top,
        "███████╗███╗   ██╗ █████╗ ██╗  ██╗███████╗\n\
         ██╔════╝████╗  ██║██╔══██╗██║ ██╔╝██╔════╝\n\
         ███████╗██╔██╗ ██║███████║█████╔╝ █████╗  \n\
         ╚════██║██║╚██╗██║██╔══██║██╔═██╗ ██╔══╝  \n\
         ███████║██║ ╚████║██║  ██║██║  ██╗███████╗\n\
         ╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝"
            .to_string(),
        width,
        height,
        Color::Green.to_rgb(),
    );

    let start = Button::new(
        0,
        -5,
        Alignment::Center,
        "START".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Start,
    );

    let exit = Button::new(
        -1,
        -3,
        Alignment::Center,
        "EXIT".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Exit,
    );

    let version = Text::new(
        0,
        -3,
        Alignment::Bottom,
        "v0.1.0".to_string(),
        width,
        height,
        Color::LightCyan.to_rgb(),
    );

    let info = Text::new(
        0,
        -2,
        Alignment::Bottom,
        "Written in Rust by ErFer7".to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    ui_scene.add_text(title);
    ui_scene.add_text(version);
    ui_scene.add_text(info);

    ui_scene.add_button(start);
    ui_scene.add_button(exit);

    return ui_scene;
}

pub fn build_paused_scene(width: u16, height: u16) -> UiScene {
    let mut ui_scene = UiScene::new(width, height);

    let title = Text::new(
        0,
        5,
        Alignment::Top,
        "██████╗  █████╗ ██╗   ██╗███████╗███████╗██████╗ \n\
         ██╔══██╗██╔══██╗██║   ██║██╔════╝██╔════╝██╔══██╗\n\
         ██████╔╝███████║██║   ██║███████╗█████╗  ██║  ██║\n\
         ██╔═══╝ ██╔══██║██║   ██║╚════██║██╔══╝  ██║  ██║\n\
         ██║     ██║  ██║╚██████╔╝███████║███████╗██████╔╝\n\
         ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝╚═════╝"
            .to_string(),
        width,
        height,
        Color::White.to_rgb(),
    );

    let resume = Button::new(
        0,
        -5,
        Alignment::Center,
        "RESUME".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Resume,
    );

    let exit = Button::new(
        -1,
        -3,
        Alignment::Center,
        "END".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::End,
    );

    ui_scene.add_text(title);
    ui_scene.add_button(resume);
    ui_scene.add_button(exit);

    return ui_scene;
}

pub fn build_game_over_scene(width: u16, height: u16) -> UiScene {
    let mut ui_scene = UiScene::new(width, height);

    let title = Text::new(
        0,
        5,
        Alignment::Top,
        "██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ██╗   ██╗███████╗██████╗ \n\
         ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██║   ██║██╔════╝██╔══██╗\n\
         ██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║██║   ██║█████╗  ██████╔╝\n\
         ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗\n\
         ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║\n\
         ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝"
            .to_string(),
        width,
        height,
        Color::LightRed.to_rgb(),
    );

    let exit = Button::new(
        0,
        -5,
        Alignment::Center,
        "MENU".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::GoToMenu,
    );

    ui_scene.add_text(title);
    ui_scene.add_button(exit);

    return ui_scene;
}
