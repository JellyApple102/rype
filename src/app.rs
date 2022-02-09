use tui::Terminal;
use tui::backend::Backend;
use rand::Rng;

pub struct App<'a> {
    pub should_quit: bool,
    pub state: GameState,
    pub focused_window: FocusedWindow,
    pub selected_game_tab: usize,
    pub selected_timer_tab: usize,
    pub game_options: Vec<&'a str>,
    pub timer_options: Vec<&'a str>,
    pub words: Vec<&'a str>,
    pub my_game_text: String,
    pub game_text: String,
    pub cursor_index: usize,
}

impl<'a> App<'a> {
    pub fn new(words: Vec<&'a str>) -> Self {
        let mut a = App {
            should_quit: false,
            state: GameState::Pre,
            focused_window: FocusedWindow::Game,
            selected_game_tab: 0,
            selected_timer_tab: 0,
            game_options: vec!["100", "1k", "5k", "10k"],
            timer_options: vec!["30", "60", "120"],
            my_game_text: "".to_string(),
            words,
            game_text: "".to_string(),
            cursor_index: 0,
        };

        a.game_text = a.gen_test();
        a
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

                self.game_text = self.gen_test();
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

                self.game_text = self.gen_test();
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

    pub fn start_game<B: Backend> (&mut self, term: &mut Terminal<B>) {
        match term.clear() {
            Ok(()) => {},
            Err(error) => {
                panic!("error clearing terminal: {:?}", error);
            }
        }
        self.state = GameState::During;
    }

    pub fn end_game(&mut self) {
        self.my_game_text = "".to_string();
        self.state = GameState::Pre;
        self.game_text = self.gen_test();
        self.cursor_index = 0;
    }

    pub fn on_char(&mut self, c: char) {
        if c == '\x08' {
            if !self.my_game_text.is_empty() {
                let chars: Vec<char> = self.my_game_text.chars().collect();
                if chars[self.cursor_index - 1] == '\0' {
                    while self.cursor_index > 0 && chars[self.cursor_index - 1] == '\0' {
                        self.my_game_text.pop();
                        self.cursor_index -= 1;
                    }
                } else {
                    self.my_game_text.pop();
                    self.cursor_index -= 1;
                }
            }
        } else if c == ' ' {
            let chars: Vec<char> = self.game_text.chars().collect();

            if chars[self.cursor_index] != ' ' {
                if self.cursor_index == 0 || chars[self.cursor_index - 1] == ' ' {
                    return;
                }

                while chars[self.cursor_index + 1] != ' ' {
                    self.my_game_text.push('\0');
                    self.cursor_index += 1;
                }
                self.my_game_text.push('\0'); // last character on word
                self.my_game_text.push('\0'); // space fill
                self.cursor_index += 2;
            } else {
                self.my_game_text.push(' ');
                self.cursor_index += 1;
            }
        } else {
            self.my_game_text.push(c);
            self.cursor_index += 1;
        }
    }

    pub fn gen_test(&mut self) -> String {
        let opt = self.game_options[self.selected_game_tab].to_string().replace("k", "000");
        let mut r: usize = opt.parse().unwrap();
        if r > self.words.len() {
            r = self.words.len();
        }

        let mut out = "".to_string();
        let mut last_word = "";
        for _ in 0..500 {
            let mut rng = rand::thread_rng();
            let rand = rng.gen_range(0..=r);

            if self.words[rand] == last_word {
                continue;
            }

            out.push_str(self.words[rand]);
            out.push(' ');
            last_word = self.words[rand];
        }

        out = out.trim().to_string();
        out
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
