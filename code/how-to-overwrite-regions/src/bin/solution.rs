use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

// ANCHOR: imports
use derive_setters::Setters;
use lipsum::lipsum;
use ratatui::{prelude::*, widgets::*};
// ANCHOR_END: imports

struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

// ANCHOR: popup
#[derive(Debug, Default, Setters)]
struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}
// ANCHOR_END: popup

fn main() -> color_eyre::Result<()> {
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
    // ANCHOR: solution
    let popup = Popup::default()
        .content("Hello world!")
        .style(Style::new().yellow())
        .title("With Clear")
        .title_style(Style::new().white().bold())
        .border_style(Style::new().red());
    frame.render_widget(popup, popup_area);
    // ANCHOR_END: solution;
}
// ANCHOR_END: ui

fn key_pressed() -> color_eyre::Result<bool> {
    Ok(event::poll(Duration::from_millis(16))?
        && matches!(event::read()?, Event::Key(_)))
}

impl Term {
    fn init() -> color_eyre::Result<Self> {
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
