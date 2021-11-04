pub struct App {
    pub should_quit: bool,
    pub state: GameState,
    pub focused_window: FocusedWindow
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            state: GameState::Pre,
            focused_window: FocusedWindow::Game
        }
    }
}

#[allow(dead_code)]
pub enum GameState {
    Pre,
    During,
    Post
}

pub enum FocusedWindow {
   GameOptions,
   TimerOptions,
   Game
}

impl FocusedWindow {
    pub fn next(self) -> FocusedWindow {
        match self {
            FocusedWindow::Game => FocusedWindow::GameOptions,
            FocusedWindow::GameOptions => FocusedWindow::TimerOptions,
            FocusedWindow::TimerOptions => FocusedWindow::Game
        }
    }

    pub fn prev(self) -> FocusedWindow {
        match self {
            FocusedWindow::Game => FocusedWindow::TimerOptions,
            FocusedWindow::GameOptions => FocusedWindow::Game,
            FocusedWindow::TimerOptions => FocusedWindow::GameOptions
        }
    }
}
