use std::{
    io,
    thread,
    sync::mpsc,
    time::{Duration, Instant}
};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, BorderType, Borders, Paragraph},
    layout::{Alignment, Constraint, Direction, Layout}
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{self, Event as CEvent, KeyCode}
};

mod ui;

enum Event<I> {
    Input(I),
    Tick
}

enum FocusedWindow {
    GameOptions,
    TimerOptions,
    Game
}

enum GameState {
    Pre,
    During,
    Post
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // check getting list of words
    // let words: Vec<&str> = include!("words.txt");
    // println!("{}", words.len());

    // basic setup
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    enable_raw_mode().expect("can run in raw mode");

    // input setup
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    let focused_window = FocusedWindow::Game;
    let game_state = GameState::Pre;

    // draw loop
    loop {
        terminal.draw(|f| {
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

            match focused_window {
                FocusedWindow::GameOptions => {
                    ui::render_game_options(f, header_chunks[0], true);
                    ui::render_timer_options(f, header_chunks[1], false);
                },
                FocusedWindow::TimerOptions => {
                    ui::render_game_options(f, header_chunks[0], false);
                    ui::render_timer_options(f, header_chunks[1], true);
                },
                FocusedWindow::Game => {
                    ui::render_game_options(f, header_chunks[0], false);
                    ui::render_timer_options(f, header_chunks[1], false);
                }
            }

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
        })?;

        // input handling
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                },
                KeyCode::Enter => {}
                _ => {}
            },
            Event::Tick => {}
        }
    }
    Ok(())
}
