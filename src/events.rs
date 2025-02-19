#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Pause,
    Resume,
    Restart,
    End,
    GoToMenu,
    Exit,
    None,
}
