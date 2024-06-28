use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use color_eyre::Result;
// ANCHOR: imports
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    symbols,
    widgets::{Block, Borders},
    Frame, Terminal,
};
// ANCHOR_END: imports

struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

/// This example shows how to use custom borders to collapse borders between widgets.
/// See https://ratatui.rs/how-to/layout/collapse-borders for more info
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

    // ANCHOR: layout
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        // use a 49/51 split instead of 50/50 to ensure that any extra space is on the right
        // side of the screen. This is important because the right side of the screen is
        // where the borders are collapsed.
        .constraints([Constraint::Percentage(49), Constraint::Percentage(51)])
        .split(frame.size());
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        // use a 49/51 split to ensure that any extra space is on the bottom
        .constraints([Constraint::Percentage(49), Constraint::Percentage(51)])
        .split(layout[1]);
    // ANCHOR_END: layout

    // ANCHOR: left_block
    frame.render_widget(
        Block::new()
            // don't render the right border because it will be rendered by the right block
            .border_set(symbols::border::PLAIN)
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .title("Left Block"),
        layout[0],
    );
    // ANCHOR_END: left_block

    // ANCHOR: top_right_block
    // top right block must render the top left border to join with the left block
    let top_right_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.horizontal_down,
        ..symbols::border::PLAIN
    };
    frame.render_widget(
        Block::new()
            .border_set(top_right_border_set)
            // don't render the bottom border because it will be rendered by the bottom block
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .title("Top Right Block"),
        sub_layout[0],
    );
    // ANCHOR_END: top_right_block

    // ANCHOR: bottom_right_block
    // bottom right block must render:
    // - top left border to join with the left block and top right block
    // - top right border to join with the top right block
    // - bottom left border to join with the left block
    let collapsed_top_and_left_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        bottom_left: symbols::line::NORMAL.horizontal_up,
        ..symbols::border::PLAIN
    };
    frame.render_widget(
        Block::new()
            .border_set(collapsed_top_and_left_border_set)
            .borders(Borders::ALL)
            .title("Bottom Right Block"),
        sub_layout[1],
    );
    // ANCHOR_END: bottom_right_block
}
// ANCHOR_END: ui

fn key_pressed() -> Result<bool> {
    Ok(event::poll(Duration::from_millis(16))? && matches!(event::read()?, Event::Key(_)))
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
