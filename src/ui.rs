use tui::{
    Frame,
    backend::Backend,
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Wrap},
    layout::{Alignment, Rect, Layout, Direction, Constraint},
    style::{Style, Color},
    text::{Spans, Span}
};

use super::App;
use super::app::FocusedWindow;
use super::app::GameState;
use super::widgets::alignedtabs::AlignedTabs;

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

    match app.state {
        GameState::Pre => {
            render_header_widgets(f, header_chunks[0], header_chunks[1], app);
            // render_timer(f, timer_chunks[1], app.current_timer);
            let timer = Paragraph::new("timer here")
                .style(Style::default().fg(Color::Blue))
                .alignment(Alignment::Left);
            f.render_widget(timer, timer_chunks[1]);

            // let typing_section = Paragraph::new("type here")
            //     // .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
            //     .alignment(Alignment::Left);
            // f.render_widget(typing_section, game_chunks[1]);
            render_typing_section(f, game_chunks[1], app);

            let footer = Paragraph::new("Footer")
                .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
                .alignment(Alignment::Center);
            f.render_widget(footer, chunks[2]);
        },
        GameState::During => {
            // render_timer(f, timer_chunks[1], app);
            let timer = Paragraph::new(format!("timer started {}", app.cursor_index))
                .style(Style::default().fg(Color::Blue))
                .alignment(Alignment::Left);
            f.render_widget(timer, timer_chunks[1]);

            // let typing_section = Paragraph::new("type here")
            //     // .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
            //     .alignment(Alignment::Left);
            // f.render_widget(typing_section, game_chunks[1]);
            render_typing_section(f, game_chunks[1], app);
        },
        GameState::Post => {}
    }
}

fn render_game_options<B: Backend>(f: &mut Frame<B>, area: Rect, focused: bool, app: &App) {
    let options = app.game_options.iter().cloned().map(Spans::from).collect();
    let mut game_options_tabs = Tabs::new(options)
        .select(app.selected_game_tab)
        .highlight_style(Style::default().fg(Color::Green));

    let mut b = Block::default()
        .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded);

    if focused {
        b = b.border_style(Style::default().fg(Color::Red));
    }

    game_options_tabs = game_options_tabs.block(b);
    f.render_widget(game_options_tabs, area);
}

fn render_timer_options<B: Backend>(f: &mut Frame<B>, area: Rect, focused: bool, app: &App) {
    let options = app.timer_options.iter().cloned().map(Spans::from).collect();
    let mut timer_options_tabs = AlignedTabs::new(options)
        .select(app.selected_timer_tab)
        .alignment(Alignment::Right)
        .highlight_style(Style::default().fg(Color::Green));

    let mut b = Block::default()
        .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded);

    if focused {
        b = b.border_style(Style::default().fg(Color::Red));
    }

    timer_options_tabs = timer_options_tabs.block(b);
    f.render_widget(timer_options_tabs, area);
}

fn render_header_widgets<B: Backend>(f: &mut Frame<B>, options_area: Rect, timer_area: Rect, app: &App) {
    match app.focused_window {
        FocusedWindow::Game => {
            render_game_options(f, options_area, false, app);
            render_timer_options(f, timer_area, false, app);
        },
        FocusedWindow::GameOptions => {
            render_game_options(f, options_area, true, app);
            render_timer_options(f, timer_area, false, app);
        },
        FocusedWindow::TimerOptions => {
            render_game_options(f, options_area, false, app);
            render_timer_options(f, timer_area, true, app);
        }
    }
}

// fn render_timer<B: Backend> (f: &mut Frame<B>, timer_area: Rect, app: &App) {
//     let timer = Paragraph::new(time.to_string())
//         .alignment(Alignment::Left);
//     f.render_widget(timer, timer_area);
// }

/*
app maybe has a Vector of 'skipped' words character indexes
check against this vector when backpace to jump back as needed
when space in middle of word store index in vector and jump to next word
fill 'skipped space' with a weird rando bs char? style/render appropriately
app would need to store cursor index
*/

fn render_typing_section<B: Backend> (f: &mut Frame<B>, typing_area: Rect, app: &mut App) {
    let text_bytes: Vec<char> = app.game_text.chars().collect();
    let my_text_bytes: Vec<char> = app.my_game_text.chars().collect();
    let mut c_index: usize = 0;
    let mut para = vec![];

    for (i, c) in my_text_bytes.iter().enumerate() {
        c_index = i + 1;
        if c == &text_bytes[i] {
            //let c = my_text_bytes[i] as char;
            let c = c.to_string();
            let s = Span::styled(c, Style::default().fg(Color::Green));
            para.push(s);
        } else if *c == '\0' {
            let c = text_bytes[i];
            let c = c.to_string();
            let s = Span::styled(c, Style::default().fg(Color::Yellow));
            para.push(s);
        } else {
            let c = text_bytes[i];
            let c = c.to_string();
            let s = Span::styled(c, Style::default().fg(Color::Red));
            para.push(s);
        }
    }

    para.push({
        let content: String = text_bytes[c_index..c_index + 1].iter().collect();
        let style = Style::default().bg(Color::DarkGray).fg(Color::Black);
        Span {
            content: content.into(),
            style,
        }
    });
    para.push({
        let content: String = text_bytes[c_index + 1..].iter().collect();
        Span {
            content: content.into(),
            style: Style::default(),
        }
    });

    let typing_section = Paragraph::new(Spans::from(para))
        .alignment(Alignment::Left)
        .wrap(Wrap{ trim: true });
    f.render_widget(typing_section, typing_area);
}
