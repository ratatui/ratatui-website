// ANCHOR: new imports
use std::panic;

use color_eyre::{
    config::HookBuilder,
    eyre::{self, bail, WrapErr},
};
// ANCHOR_END: new imports
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

// ANCHOR: modules
mod tui;
// ANCHOR_END: modules

// ANCHOR: main
fn main() -> color_eyre::Result<()> {
    install_hooks()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
// ANCHOR_END: main

// ANCHOR: install_hooks
/// This replaces the standard color_eyre panic and error hooks with hooks that
/// restore the terminal before printing the panic or error.
pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();

    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        tui::restore().unwrap();
        panic_hook(panic_info);
    }));

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            tui::restore().unwrap();
            eyre_hook(error)
        },
    ))?;

    Ok(())
}
// ANCHOR_END: install_hooks

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

// ANCHOR: impl App
impl App {
    // ANCHOR: run
    pub fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
    ) -> color_eyre::Result<()> {
        while self.running_state != RunningState::Finished {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.update().wrap_err("update failed")?;
        }
        Ok(())
    }
    // ANCHOR_END: run

    // ANCHOR: render_frame
    fn render_frame(&self, frame: &mut Frame) {
        let counter_text = format!("Counter: {}", self.counter);
        let instruction_text = "Decrement: <Left> Increment: <Right> Quit: <Q>";

        let area = frame.size();
        let block = Block::default()
            .title("Basic Counter App")
            .borders(Borders::ALL);
        let inner_area = block.inner(area);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(inner_area);
        frame.render_widget(block, area);
        frame.render_widget(Paragraph::new(counter_text), layout[0]);
        frame.render_widget(Paragraph::new(instruction_text), layout[1]);
    }
    // ANCHOR_END: render_frame

    // ANCHOR: update
    fn update(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key_event) => {
                self.handle_key_event(key_event).wrap_err_with(|| {
                    format!("handling key event failed:\n{key_event:#?}")
                })
            }
            _ => Ok(()),
        }
    }
    // ANCHOR_END: update

    // ANCHOR: handle_key_event
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
    ) -> color_eyre::Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.running_state = RunningState::Finished;
            }
            KeyCode::Left => {
                self.counter -= 1;
            }
            KeyCode::Right => {
                self.counter += 1;
                if self.counter > 2 {
                    bail!("counter overflow");
                }
            }
            _ => {}
        }
        Ok(())
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

        let completed_frame = terminal
            .draw(|frame| app.render_frame(frame))
            .expect("draw");

        assert_eq!(
            *completed_frame.buffer,
            Buffer::with_lines(vec![
                "┌Basic Counter App───────────────────────────────┐",
                "│Counter: 0                                      │",
                "│Decrement: <Left> Increment: <Right> Quit: <Q>  │",
                "└────────────────────────────────────────────────┘",
            ])
        );
    }
    // ANCHOR_END: render_frame test

    // ANCHOR: handle_key_event test
    #[test]
    fn handle_key_event() -> color_eyre::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into())?;
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into())?;
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into())?;
        assert_eq!(app.running_state, RunningState::Finished);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn handle_key_event_panic() {
        let mut app = App::default();
        let _ = app.handle_key_event(KeyCode::Left.into());
    }

    #[test]
    fn handle_key_event_overflow() {
        let mut app = App::default();
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert_eq!(
            app.handle_key_event(KeyCode::Right.into())
                .unwrap_err()
                .to_string(),
            "counter overflow"
        );
    }
    // ANCHOR_END: handle_key_event test
}
// ANCHOR_END: tests
