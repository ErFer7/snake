use crate::{
    button::Button,
    cell::Color,
    cell_matrix::CellMatrix,
    events::Event,
    global_context::GameplayContext,
    scene::{ButtonInteractableScene, Scene, UpdateResult},
    terminal::Terminal,
    text::{Alignment, Text},
    world::generate_walls,
    VERSION,
};

pub struct UiScene {
    name: String,
    cell_matrix: CellMatrix,
    texts: Vec<Text>,
    buttons: Vec<Button>,
    selected_button: usize,
}

impl Scene for UiScene {
    fn new(name: String, width: u16, height: u16) -> Self {
        let mut ui_cell_matrix = CellMatrix::new(width, height);
        generate_walls(&mut ui_cell_matrix, width, height);

        UiScene {
            name,
            cell_matrix: ui_cell_matrix,
            texts: Vec::new(),
            buttons: Vec::new(),
            selected_button: 0,
        }
    }

    fn name(&self) -> String {
        return self.name.clone();
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
                return UpdateResult::new(event, gameplay_context);
            }
            Some(termion::event::Key::Esc) => {
                return UpdateResult::new(Event::Exit, gameplay_context);
            }
            _ => (),
        }

        return UpdateResult::none(gameplay_context);
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
    let mut ui_scene = UiScene::new("main_menu".to_string(), width, height);

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
        0,
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
        VERSION.to_string(),
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
    let mut ui_scene = UiScene::new("paused".to_string(), width, height);

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

    let restart = Button::new(
        0,
        -3,
        Alignment::Center,
        "RESTART".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Restart,
    );

    let exit = Button::new(
        0,
        -1,
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
    ui_scene.add_button(restart);
    ui_scene.add_button(exit);

    return ui_scene;
}

pub fn build_game_over_scene(width: u16, height: u16) -> UiScene {
    let mut ui_scene = UiScene::new("game_over".to_string(), width, height);

    let title = Text::new(
        0,
        5,
        Alignment::Top,
        " ██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ██╗   ██╗███████╗██████╗ \n\
         ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██║   ██║██╔════╝██╔══██╗\n\
         ██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║██║   ██║█████╗  ██████╔╝\n\
         ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗\n\
         ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║\n\
         \x20╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝"
            .to_string(),
        width,
        height,
        Color::LightRed.to_rgb(),
    );

    let restart = Button::new(
        0,
        -3,
        Alignment::Center,
        "RESTART".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::Restart,
    );

    let menu = Button::new(
        0,
        -1,
        Alignment::Center,
        "MENU".to_string(),
        width,
        height,
        Color::White.to_rgb(),
        Color::LightGreen.to_rgb(),
        Event::GoToMenu,
    );

    ui_scene.add_text(title);
    ui_scene.add_button(restart);
    ui_scene.add_button(menu);

    return ui_scene;
}
