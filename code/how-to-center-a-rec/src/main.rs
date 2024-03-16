use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Paragraph},
    {Frame, Terminal},
};

use std::io::stderr;

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui(f))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('q')
                {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

fn ui(frame: &mut Frame) {
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::DOUBLE)
        .style(Style::default().fg(Color::Green));

    let statement = Paragraph::new("This is a popup block ([q] to quit)")
        .block(popup_block);

    let area = centered_rect(50, 50, frame.size());

    frame.render_widget(statement, area);
}

// ANCHOR: centered_rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
// ANCHOR_END: centered_rect

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: test_function
    #[test]
    fn test_centered_rect() {
        let rect = Rect::new(0, 0, 200, 150);
        let expected_rect = Rect::new(74, 18, 50, 113);

        let actual_rect = centered_rect(25, 75, rect);
        assert_eq!(actual_rect, expected_rect);
    }
    // ANCHOR_END: test_function
}
