use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if key_pressed()? {
            return Ok(());
        }
    }
}

fn key_pressed() -> Result<bool> {
    Ok(event::poll(Duration::from_millis(16))? && matches!(event::read()?, Event::Key(_)))
}

// ANCHOR: draw
fn draw(frame: &mut Frame) {
    // create a layout that splits the screen into 2 equal columns and the right column
    // into 2 equal rows
    let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
    let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2]).areas(right);

    frame.render_widget(Block::bordered().title("Left Block"), left);
    frame.render_widget(Block::bordered().title("Top Right Block"), top_right);
    frame.render_widget(Block::bordered().title("Bottom Right Block"), bottom_right);
}
// ANCHOR_END: draw
