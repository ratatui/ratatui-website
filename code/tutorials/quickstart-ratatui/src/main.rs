use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    terminal.draw(|frame| {
        let block = Block::bordered().title("Welcome");
        let greeting = Paragraph::new("Hello, Ratatui! 🐭")
            .centered()
            .on_cyan()
            .block(block);
        frame.render_widget(greeting, frame.area());
    })?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    ratatui::restore();
    Ok(())
}
