use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

pub mod grid;
pub mod layout;

const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(100);

fn main() -> color_eyre::Result<()> {
    // -- snip --
    let mut terminal = ratatui::init();
    let mut is_running = true;
    while is_running {
        terminal.draw(|frame| {
            let area = frame.area();
            let title = Line::from("Ratatui Simple Template")
                .bold()
                .blue()
                .centered();
            let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
            frame.render_widget(
                Paragraph::new(text)
                    .block(Block::bordered().title(title))
                    .centered(),
                frame.area(),
            );
        })?;

        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                on_key_event(key, &mut is_running)
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        };
    }
    Ok(())
}

fn on_key_event(key: KeyEvent, is_running: &mut bool) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
            *is_running = false;
        }
        // Add other key handlers here.
        _ => {}
    }
}
