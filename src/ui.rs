use tui::{
    Frame,
    backend::Backend,
    widgets::{Block, BorderType, Borders, Paragraph},
    layout::{Alignment, Rect},
    style::{Style, Color}
};

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

fn render_game_options<B: Backend>(f: &mut Frame<B>, area: Rect, focused: bool) {
    let mut game_options_tabs = Paragraph::new("game options here")
        .alignment(Alignment::Left);

    let mut b = Block::default()
        .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded);

    if focused {
        b = b.border_style(Style::default().fg(Color::Red));
    }

    game_options_tabs = game_options_tabs.block(b);
    f.render_widget(game_options_tabs, area);
}

fn render_timer_options<B: Backend>(f: &mut Frame<B>, area: Rect, focused: bool) {
    let mut timer_options_tabs = Paragraph::new("timer options here")
        .alignment(Alignment::Right);

    let mut b = Block::default()
        .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded);

    if focused {
        b = b.border_style(Style::default().fg(Color::Red));
    }

    timer_options_tabs = timer_options_tabs.block(b);
    f.render_widget(timer_options_tabs, area);
}

pub fn render_header_widgets<B: Backend>(f: &mut Frame<B>, options_area: Rect, timer_area: Rect, focused: &FocusedWindow) {
    match focused {
        FocusedWindow::Game => {
            render_game_options(f, options_area, false);
            render_timer_options(f, timer_area, false);
        },
        FocusedWindow::GameOptions => {
            render_game_options(f, options_area, true);
            render_timer_options(f, timer_area, false);
        },
        FocusedWindow::TimerOptions => {
            render_game_options(f, options_area, false);
            render_timer_options(f, timer_area, true);
        }
    }
}
