use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event};
// ANCHOR: imports
use ratatui::{
    layout::{Constraint, Layout},
    symbols,
    widgets::{Block, Borders},
    DefaultTerminal, Frame,
};
// ANCHOR_END: imports

/// This example shows how to use custom borders to collapse borders between widgets.
/// See https://ratatui.rs/how-to/layout/collapse-borders for more info
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

    // ANCHOR: layout
    // use a 49/51 split instead of 50/50 to ensure that any extra space is on the right
    // side of the screen. This is important because the right side of the screen is
    // where the borders are collapsed.
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(49), Constraint::Percentage(51)])
            .areas(frame.area());
    // use a 49/51 split to ensure that any extra space is on the bottom
    let [top_right, bottom_right] =
        Layout::vertical([Constraint::Percentage(49), Constraint::Percentage(51)]).areas(right);
    // ANCHOR_END: layout

    // ANCHOR: left_block
    let left_block = Block::new()
        // don't render the right border because it will be rendered by the right block
        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
        .title("Left Block");
    // ANCHOR_END: left_block

    // ANCHOR: top_right_block
    // top right block must render the top left border to join with the left block
    let top_right_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.horizontal_down,
        ..symbols::border::PLAIN
    };
    let top_right_block = Block::new()
        .border_set(top_right_border_set)
        // don't render the bottom border because it will be rendered by the bottom block
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .title("Top Right Block");
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
    let bottom_right_block = Block::new()
        .border_set(collapsed_top_and_left_border_set)
        .borders(Borders::ALL)
        .title("Bottom Right Block");
    // ANCHOR_END: bottom_right_block

    // ANCHOR: render
    frame.render_widget(left_block, left);
    frame.render_widget(top_right_block, top_right);
    frame.render_widget(bottom_right_block, bottom_right);
    // ANCHOR_END: render
}
// ANCHOR_END: draw
