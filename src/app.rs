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

    pub fn cycle_focus_forward(&mut self) {
        match self.focused_window {
            FocusedWindow::Game => self.focused_window = FocusedWindow::GameOptions,
            FocusedWindow::GameOptions => self.focused_window = FocusedWindow::TimerOptions,
            FocusedWindow::TimerOptions => self.focused_window = FocusedWindow::Game
        }
    }

    pub fn cycle_focus_backward(&mut self) {
        match self.focused_window {
            FocusedWindow::Game => self.focused_window = FocusedWindow::TimerOptions,
            FocusedWindow::GameOptions => self.focused_window = FocusedWindow::Game,
            FocusedWindow::TimerOptions => self.focused_window = FocusedWindow::GameOptions
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
