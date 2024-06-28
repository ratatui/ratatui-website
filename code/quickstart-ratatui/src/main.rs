use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Margin},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    terminal.draw(|f| {
        let widget = Paragraph::new("Hello world!")
            .alignment(Alignment::Center)
            .block(Block::new().borders(Borders::ALL));

        let area = f.size().inner(Margin {
            horizontal: 2,
            vertical: 2,
        });

        f.render_widget(widget, area);
    })?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
