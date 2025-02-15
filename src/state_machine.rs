#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    MainMenu,
    Starting,
    Gameplay,
    Paused,
    GameOver,
    Exit,
    Invalid,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Pause,
    Resume,
    End,
    GoToMenu,
    Exit,
    None,
}

impl State {
    pub fn transition(self, event: Event) -> State {
        match (self, event) {
            (State::MainMenu, Event::Start) => State::Starting,
            (State::Gameplay, Event::Pause) => State::Paused,
            (State::Paused, Event::Resume) => State::Gameplay,
            (State::Gameplay, Event::End) | (State::Paused, Event::End) => State::GameOver,
            (State::GameOver, Event::GoToMenu) => State::MainMenu,
            (_, Event::Exit) => State::Exit,
            (State::Starting, Event::None) => State::Gameplay,
            _ => State::Invalid,
        }
    }
}
