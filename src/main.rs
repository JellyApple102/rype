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
        // f.render_widget(main_chunk, f.size());

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

        f.render_widget(main_chunk, f.size());

        let header = Paragraph::new("Header")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // main block
        // let block = Block::default()
        //     .title("")
        //     .borders(Borders::ALL)
        //     .border_type(BorderType::Rounded);
        // f.render_widget(block, chunks[1]);

        let game_chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(chunks[1].width - (chunks[1].width - 10))
            .vertical_margin(chunks[1].height - (chunks[1].height - 5))
            .constraints(
                [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks[1]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        f.render_widget(block, game_chunks[0]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        f.render_widget(block, game_chunks[1]);

        let footer = Paragraph::new("Footer")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[2]);
    })?;
    Ok(())
}
