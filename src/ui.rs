use tui::{
    Frame,
    backend::Backend,
    widgets::{Block, BorderType, Borders, Paragraph},
    layout::{Alignment, Rect},
    style::{Style, Color}
};

pub fn render_game_options<B: Backend>(f: &mut Frame<B>, area: Rect, focused: bool) {
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

pub fn render_timer_options<B: Backend>(f: &mut Frame<B>, area: Rect, focused: bool) {
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
