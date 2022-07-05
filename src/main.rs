use std::{
    io,
    thread,
    sync::mpsc,
    time::{Duration, Instant}
};
use tui::{
    Terminal,
    backend::CrosstermBackend
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{self, Event as CEvent, KeyCode}
};

mod ui;
mod app;
mod widgets;

use app::App;
use app::GameState;
use app::FocusedWindow;

pub enum Event<I> {
    Input(I),
    TimerUpdate(u16),
    Tick
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // check getting list of words
    // this kills compile time - should chek if i can do anything about that
    let word_list: Vec<&str> = include!("words.txt");
    println!("{}", word_list.len());

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

    let mut app = App::new(word_list);

    // draw loop
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match rx.recv()? {
            Event::Input(event) => match app.state {
                GameState::Pre => match event.code {
                    KeyCode::Esc => {
                        app.should_quit = true;
                    },
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    KeyCode::Tab => app.cycle_focus_forward(),
                    KeyCode::BackTab => app.cycle_focus_backward(),
                    KeyCode::Left => app.cycle_tab_backward(),
                    KeyCode::Right => app.cycle_tab_forward(),
                    KeyCode::Enter => if let FocusedWindow::Game = app.focused_window {
                        app.start_game(&mut terminal);
                    }
                    _ => {}
                },
                GameState::During => match event.code {
                    KeyCode::Esc => {
                        app.should_quit = true;
                    },
                    KeyCode::Char(c) => app.on_char(c),
                    KeyCode::Backspace => app.on_char('\x08'),
                    KeyCode::Enter => app.end_game(),
                    _ => {}
                },
                GameState::Post => match event.code {
                    KeyCode::Esc => {
                        app.should_quit = true;
                    },
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    KeyCode::Enter => {},
                    _ => {}
                }
            },
            Event::Tick => {},
            _ => {}
        }

        if app.should_quit {
            disable_raw_mode()?;
            terminal.clear()?;
            terminal.show_cursor()?;
            break;
        }
    }
    Ok(())
}
