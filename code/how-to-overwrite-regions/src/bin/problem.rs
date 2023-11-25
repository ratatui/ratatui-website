use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
// ANCHOR: imports
use lipsum::lipsum;
use ratatui::{prelude::*, widgets::*};
// ANCHOR_END: imports

struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

fn main() -> anyhow::Result<()> {
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
    let area = frame.size();
    let background_text = Paragraph::new(lipsum(1000))
        .wrap(Wrap { trim: true })
        .light_blue()
        .italic()
        .on_black();
    frame.render_widget(background_text, area);

    // take up a third of the screen vertically and half horizontally
    let popup_area = Rect {
        x: area.width / 4,
        y: area.height / 3,
        width: area.width / 2,
        height: area.height / 3,
    };
    let bad_popup = Paragraph::new("Hello world!")
        .wrap(Wrap { trim: true })
        .style(Style::new().yellow())
        .block(
            Block::new()
                .title("Without Clear")
                .title_style(Style::new().white().bold())
                .borders(Borders::ALL)
                .border_style(Style::new().red()),
        );
    frame.render_widget(bad_popup, popup_area);
}
// ANCHOR_END: ui

fn key_pressed() -> anyhow::Result<bool> {
    Ok(event::poll(Duration::from_millis(16))? && matches!(event::read()?, Event::Key(_)))
}

impl Term {
    fn init() -> anyhow::Result<Self> {
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
