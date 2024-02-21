use crates_tui::errors;
use crates_tui::events;
use crates_tui::tui;

use color_eyre::Result;
use events::{Event, Events};
use ratatui::prelude::*;
use ratatui::widgets::*;
use tui::Tui;

// ANCHOR: full_app

// ANCHOR: app
pub struct App {
    quit: bool,
    frame_count: usize,
    last_key_event: Option<crossterm::event::KeyEvent>,
}
// ANCHOR_END: app

impl App {
    // ANCHOR: app_new
    pub fn new() -> Self {
        let quit = false;
        let frame_count = 0;
        let last_key_event = None;
        Self {
            quit,
            frame_count,
            last_key_event,
        }
    }
    // ANCHOR_END: app_new

    // ANCHOR: app_run
    pub async fn run(
        &mut self,
        mut tui: Tui,
        mut events: Events,
    ) -> Result<()> {
        loop {
            if let Some(e) = events.next().await {
                self.handle_event(e, &mut tui)?
            }
            if self.should_quit() {
                break;
            }
        }
        Ok(())
    }
    // ANCHOR_END: app_run

    // ANCHOR: app_handle_event
    fn handle_event(&mut self, e: Event, tui: &mut Tui) -> Result<()> {
        use crossterm::event::Event as CrosstermEvent;
        use crossterm::event::KeyCode;
        match e {
            Event::Crossterm(CrosstermEvent::Key(key)) => {
                self.last_key_event = Some(key);
                if key.code == KeyCode::Esc {
                    self.quit()
                }
            }
            Event::Render => self.draw(tui)?,
            _ => (),
        };
        Ok(())
    }
    // ANCHOR_END: app_handle_event

    // ANCHOR: app_draw
    fn draw(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            self.frame_count = frame.count();
            frame.render_stateful_widget(AppWidget, frame.size(), self);
        })?;
        Ok(())
    }
    // ANCHOR_END: app_draw

    // ANCHOR: app_quit
    fn should_quit(&self) -> bool {
        self.quit
    }

    fn quit(&mut self) {
        self.quit = true
    }
    // ANCHOR_END: app_quit
}

// ANCHOR: app_default
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
// ANCHOR_END: app_default

// ANCHOR: app_widget
struct AppWidget;
// ANCHOR_END: app_widget

// ANCHOR: app_statefulwidget
impl StatefulWidget for AppWidget {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Paragraph::new(format!("frame counter: {}", state.frame_count))
            .render(area, buf);

        if let Some(key) = state.last_key_event {
            Paragraph::new(format!("last key event: {:?}", key.code))
                .right_aligned()
                .render(area, buf);
        }
    }
}
// ANCHOR_END: app_statefulwidget

// ANCHOR_END: full_app

// ANCHOR: main
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    errors::install_hooks()?;
    let tui = tui::init()?;
    let events = events::Events::new();

    App::new().run(tui, events).await?;

    tui::restore()?;

    Ok(())
}
// ANCHOR_END: main
