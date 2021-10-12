use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::layout::{Alignment, Constraint, Direction, Layout};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let main_chunk = Block::default()
            .title("rype")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        f.render_widget(main_chunk, f.size());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3)
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

        let header = Paragraph::new("Header")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        let timer = Paragraph::new("timer here")
            .alignment(Alignment::Left);
        f.render_widget(timer, timer_chunks[1]);

        let typing_section = Paragraph::new("type here")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
            .alignment(Alignment::Left);
        f.render_widget(typing_section, game_chunks[1]);

        // let padding_block = Block::default()
        //     .borders(Borders::ALL)
        //     .border_type(BorderType::Rounded);
        // f.render_widget(padding_block, game_chunks[2]);

        let footer = Paragraph::new("Footer")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    })?;
    Ok(())
}
