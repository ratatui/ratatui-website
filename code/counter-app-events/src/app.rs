use std::sync::mpsc;

use color_eyre::{eyre::bail, Result};
use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, Title},
        *,
    },
};
use tracing::{debug, info, instrument};

use crate::{events::AppEvent, logging::LogEvents};

// ANCHOR: app
#[derive(Debug, Default)]
pub struct App {
    pub counter: u8,
    pub event_tx: Option<mpsc::Sender<AppEvent>>,
    pub logs: LogEvents,
}
// ANCHOR_END: app

// ANCHOR: impl App
impl App {
    pub fn new(logs: LogEvents) -> App {
        Self {
            logs,
            ..Default::default()
        }
    }

    #[instrument(skip(self))]
    pub fn handle_key_press(&mut self, key_event: KeyEvent) -> Result<()> {
        debug!(?key_event, "handling key event");
        use crossterm::event::KeyCode::*;
        match key_event.code {
            Esc | Char('q') => self.send_event(AppEvent::Quit),
            Left | Char('j') => self.decrement_counter()?,
            Right | Char('k') => self.increment_counter()?,
            _ => {}
        }
        Ok(())
    }

    pub fn decrement_counter(&mut self) -> Result<()> {
        info!("decrementing counter");
        self.counter -= 1;
        self.send_event(AppEvent::Redraw);
        Ok(())
    }

    pub fn increment_counter(&mut self) -> Result<()> {
        info!("incrementing counter");
        self.counter += 1;
        if self.counter > 20 {
            bail!("counter overflow");
        }
        self.send_event(AppEvent::Redraw);
        Ok(())
    }

    #[instrument(skip(self))]
    fn send_event(&self, event: AppEvent) {
        debug!(?event, "sending event");
        if let Some(sender) = &self.event_tx {
            // we can safely ignore the error here because it just indicates that the receiver
            // has been dropped, which is fine for our use case
            let _ = sender.send(event);
        }
    }
}
// ANCHOR_END: impl App

// ANCHOR: impl Widget
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);
        (&block).render(area, buf);
        let area = block.inner(area);

        let [counter_area, logs_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                .areas(area);

        Paragraph::new(Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ]))
        .centered()
        .render(counter_area, buf);

        self.logs.render(logs_area, buf);
    }
}
// ANCHOR_END: impl Widget

// ANCHOR: tests
#[cfg(test)]
mod tests {
    // ANCHOR: render test
    use super::*;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        // note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
        assert_eq!(buf, expected);
    }
    // ANCHOR_END: render test

    // ANCHOR: handle_key_event test
    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.increment_counter().unwrap();
        assert_eq!(app.counter, 1);

        app.decrement_counter().unwrap();
        assert_eq!(app.counter, 0);
    }
    // ANCHOR_END: handle_key_event test

    // ANCHOR: handle_key_event_panic
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn handle_key_event_panic() {
        let mut app = App::default();
        let _ = app.decrement_counter();
    }
    // ANCHOR_END: handle_key_event_panic

    // ANCHOR: handle_key_event_overflow
    #[test]
    fn handle_key_event_overflow() {
        let mut app = App::default();
        assert!(app.increment_counter().is_ok());
        assert!(app.increment_counter().is_ok());
        assert_eq!(
            app.increment_counter().unwrap_err().to_string(),
            "counter overflow"
        );
    }
    // ANCHOR_END: handle_key_event_overflow
}
// ANCHOR_END: tests
