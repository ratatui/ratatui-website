// ANCHOR: imports
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, Title},
        *,
    },
};
// ANCHOR_END: imports

// ANCHOR: modules
mod tui;
// ANCHOR_END: modules

// ANCHOR: app
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    running_state: RunningState,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Finished,
}
// ANCHOR_END: app

// ANCHOR: main
fn main() -> std::io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
// ANCHOR_END: main

// ANCHOR: impl App
impl App {
    // ANCHOR: run
    /// runs the application's main loop until the user quits
    pub fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
    ) -> io::Result<()> {
        while !self.is_finished() {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.update()?;
        }
        Ok(())
    }

    fn is_finished(&mut self) -> bool {
        self.running_state == RunningState::Finished
    }

    fn finish(&mut self) {
        self.running_state = RunningState::Finished;
    }
    // ANCHOR_END: run

    // ANCHOR: render_frame
    /// renders a single frame of the application to the terminal
    fn render_frame(&self, frame: &mut Frame) {
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
                    .position(Position::Bottom)
                    .alignment(Alignment::Center),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);
        frame.render_widget(
            Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(block),
            frame.size(),
        );
    }
    // ANCHOR_END: render_frame

    // ANCHOR: update
    /// updates the application's state based on user input
    fn update(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        };
        Ok(())
    }
    // ANCHOR_END: update

    // ANCHOR: handle_key_event
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        }
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.finish(),
            KeyCode::Left => self.increment(),
            KeyCode::Right => self.decrement(),
            _ => {}
        }
    }

    fn decrement(&mut self) {
        self.counter += 1;
    }

    fn increment(&mut self) {
        self.counter -= 1;
    }
    // ANCHOR_END: handle_key_event
}
// ANCHOR_END: impl App

// ANCHOR: tests
#[cfg(test)]
mod tests {
    // ANCHOR: render_frame test
    use super::*;
    use ratatui::backend::TestBackend;

    #[test]
    fn render_frame() {
        let app = App::default();
        let backend = TestBackend::new(50, 4);
        let mut terminal = Terminal::new(backend).expect("terminal");

        terminal
            .draw(|frame| app.render_frame(frame))
            .expect("draw");

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

        terminal.backend().assert_buffer(&expected);
    }
    // ANCHOR_END: render_frame test

    // ANCHOR: handle_key_event test
    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.running_state, RunningState::Finished);

        Ok(())
    }
    // ANCHOR_END: handle_key_event test
}
// ANCHOR_END: tests
