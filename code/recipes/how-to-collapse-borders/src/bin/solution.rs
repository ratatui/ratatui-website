use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event};
// ANCHOR: imports
use ratatui::{
    layout::{Constraint, Layout, Spacing},
    symbols::merge::MergeStrategy,
    widgets::Block,
    DefaultTerminal, Frame,
};
// ANCHOR_END: imports

/// This example shows how to use the new Ratatui v0.30 border merging feature to collapse borders
/// between widgets.
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

fn draw(frame: &mut Frame) {
    // create a layout that splits the screen into 2 equal columns and the right column
    // into 2 equal rows

    // ANCHOR: layout
    // Use Spacing::Overlap(1) to make the borders overlap
    let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .areas(frame.area());
    let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .areas(right);
    // ANCHOR_END: layout

    // ANCHOR: blocks
    // Use merge_borders(MergeStrategy::Exact) to automatically handle border merging
    let left_block = Block::bordered()
        .title("Left Block")
        .merge_borders(MergeStrategy::Exact);

    let top_right_block = Block::bordered()
        .title("Top Right Block")
        .merge_borders(MergeStrategy::Exact);

    let bottom_right_block = Block::bordered()
        .title("Bottom Right Block")
        .merge_borders(MergeStrategy::Exact);
    // ANCHOR_END: blocks

    frame.render_widget(left_block, left);
    frame.render_widget(top_right_block, top_right);
    frame.render_widget(bottom_right_block, bottom_right);
}
