pub struct App<'a> {
    pub should_quit: bool,
    pub state: GameState,
    pub focused_window: FocusedWindow,
    pub selected_game_tab: usize,
    pub selected_timer_tab: usize,
    pub game_options: Vec<&'a str>,
    pub timer_options: Vec<&'a str>
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            should_quit: false,
            state: GameState::Pre,
            focused_window: FocusedWindow::Game,
            selected_game_tab: 0,
            selected_timer_tab: 0,
            game_options: vec!["100", "1k", "5k", "10k"],
            timer_options: vec!["30", "60", "120"]
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

    pub fn cycle_tab_forward(&mut self) {
        match self.focused_window {
            FocusedWindow::GameOptions => {
                let tabs_length = self.game_options.len();
                if self.selected_game_tab + 1 < tabs_length {
                    self.selected_game_tab += 1;
                } else {
                    self.selected_game_tab = 0;
                }
            },
            FocusedWindow::TimerOptions => {
                let tabs_length = self.timer_options.len();
                if self.selected_timer_tab + 1 < tabs_length {
                    self.selected_timer_tab += 1;
                } else {
                    self.selected_timer_tab = 0;
                }
            },
            _ => {}
        }
    }

    pub fn cycle_tab_backward(&mut self) {
        match self.focused_window {
            FocusedWindow::GameOptions => {
                let tabs_length = self.game_options.len();
                if self.selected_game_tab > 0 {
                    self.selected_game_tab -= 1;
                } else {
                    self.selected_game_tab = tabs_length - 1;
                }
            },
            FocusedWindow::TimerOptions => {
                let tabs_length = self.timer_options.len();
                if self.selected_timer_tab > 0 {
                    self.selected_timer_tab -= 1;
                } else {
                    self.selected_timer_tab = tabs_length - 1;
                }
            },
            _ => {}
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
