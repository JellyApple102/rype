use tui::{
    Frame,
    backend::Backend,
    widgets::{Block, BorderType, Borders, Paragraph},
    layout::{Alignment, Rect, Layout, Direction, Constraint},
    style::{Style, Color}
};

use super::App;
use super::app::FocusedWindow;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let title_chunk = Block::default()
        .title("rype")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(title_chunk, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
            Constraint::Length(3), // header chunk
            Constraint::Min(0),    // game chunks
            Constraint::Length(3)  // footer chunk
            ].as_ref()
        )
        .split(f.size());

    let game_chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(chunks[1].width / 5)
        .vertical_margin(chunks[1].height / 5)
        .constraints(
            [
            Constraint::Ratio(1, 3), // timer chunk
            Constraint::Ratio(1, 3), // typing chunk
            Constraint::Ratio(1, 3)  // padding chunk, not visible
            ].as_ref()
        )
        .split(chunks[1]);

    let timer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Min(0),   // padding chunk, not visible
            Constraint::Length(1) // timer chunk
            ].as_ref()
        )
        .split(game_chunks[0]);

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Percentage(50),
            Constraint::Percentage(50)
            ].as_ref()
        )
        .split(chunks[0]);

    render_header_widgets(f, header_chunks[0], header_chunks[1], app);

    let typing_section = Paragraph::new("type here")
        // .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .alignment(Alignment::Left);
    f.render_widget(typing_section, game_chunks[1]);

    let timer = Paragraph::new("timer here")
        .alignment(Alignment::Left);
    f.render_widget(timer, timer_chunks[1]);

    let footer = Paragraph::new("Footer")
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
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

pub fn render_header_widgets<B: Backend>(f: &mut Frame<B>, options_area: Rect, timer_area: Rect, app: &App) {
    match app.focused_window {
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
