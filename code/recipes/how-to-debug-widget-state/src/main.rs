//! Demonstrates how to debug widget state in a Rust application by showing a debug view of the state.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Text,
    widgets::Widget,
    DefaultTerminal, Frame,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
struct AppState {
    show_debug: bool,
    form: Form,
}

#[derive(Debug, Default)]
struct Form {
    name: String,
    age: u8,
}

impl Widget for &Form {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [name, age] = Layout::vertical([Constraint::Length(1); 2]).areas(area);
        format!("Name: {}", self.name).render(name, buf);
        format!("Age: {}", self.age).render(age, buf);
    }
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut state = AppState::default();
    loop {
        terminal.draw(|frame| render(frame, &state))?;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('d') => state.show_debug = !state.show_debug, // Toggle debug view
                    KeyCode::Char('n') => state.form.name.push('a'), // Simulate user input
                    KeyCode::Char('a') => state.form.age += 1,       // Simulate user input
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, state: &AppState) {
    let debug_width = u16::from(state.show_debug);
    let [main, debug] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(debug_width)])
        .areas(frame.area());
    frame.render_widget(&state.form, main);

    if state.show_debug {
        let debug_text = Text::from(format!("state: {state:#?}"));
        frame.render_widget(debug_text, debug);
    }
}
