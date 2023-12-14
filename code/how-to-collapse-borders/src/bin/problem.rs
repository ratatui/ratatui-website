use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use color_eyre::Result;
use crossterm::{
    event::{self, Event},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

fn main() -> Result<()> {
    let mut term = Term::init()?;
    loop {
        term.terminal.draw(ui)?;
        if key_pressed()? {
            break;
        }
    }
    Ok(())
}

// ANCHOR: ui
fn ui(frame: &mut Frame) {
    // create a layout that splits the screen into 2 equal columns and the right column
    // into 2 equal rows
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[1]);

    frame.render_widget(
        Block::new().borders(Borders::ALL).title("Left Block"),
        layout[0],
    );

    frame.render_widget(
        Block::new().borders(Borders::ALL).title("Top Right Block"),
        sub_layout[0],
    );

    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .title("Bottom Right Block"),
        sub_layout[1],
    );
}
// ANCHOR_END: ui

fn key_pressed() -> Result<bool> {
    Ok(event::poll(Duration::from_millis(16))?
        && matches!(event::read()?, Event::Key(_)))
}

impl Term {
    fn init() -> Result<Self> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self { terminal })
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}
