use crates_tui::errors;
use crates_tui::events;
use crates_tui::tui;

use color_eyre::Result;
use events::{Event, Events};
use itertools::Itertools;
use ratatui::prelude::*;
use ratatui::widgets::*;
use tui::Tui;

// ANCHOR: full_app

// ANCHOR: mode
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    #[default]
    Prompt,
    Results,
}
// ANCHOR_END: mode

// ANCHOR: app
pub struct App {
    quit: bool,
    last_key_event: Option<crossterm::event::KeyEvent>,
    mode: Mode, // new
}
// ANCHOR_END: app

impl App {
    // ANCHOR: app_new
    pub fn new() -> Self {
        let quit = false;
        let mode = Mode::default();
        let last_key_event = None;
        Self {
            quit,
            mode,
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
                    match self.mode {
                        Mode::Prompt => self.switch_mode(Mode::Results),
                        Mode::Results => self.quit(),
                    }
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
            frame.render_stateful_widget(AppWidget, frame.size(), self);
        })?;
        Ok(())
    }
    // ANCHOR_END: app_draw

    // ANCHOR: app_switch_mode
    fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
    // ANCHOR_END: app_switch_mode

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
        let [last_key_event, results, prompt] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(0),
            Constraint::Length(5),
        ])
        .areas(area);

        let table = state.results();
        Widget::render(table, results, buf);

        let (block, paragraph) = state.prompt();
        block.render(prompt, buf);
        paragraph.render(
            prompt.inner(&Margin {
                horizontal: 2,
                vertical: 2,
            }),
            buf,
        );

        if let Some(key) = state.last_key_event {
            Paragraph::new(format!("last key event: {:?}", key.code))
                .right_aligned()
                .render(last_key_event, buf);
        }
    }
}
// ANCHOR_END: app_statefulwidget

impl App {
    // ANCHOR: app_results_table_widget
    fn results(&self) -> Table<'_> {
        let widths = [
            Constraint::Length(15),
            Constraint::Min(0),
            Constraint::Length(20),
        ];

        let rows = vec![
            ["hyper", "Fast and safe HTTP implementation", "1000000"],
            ["serde", "Rust data structures", "1500000"],
            ["tokio", "non-blocking I/O platform", "1300000"],
            ["rand", "random number generation", "900000"],
            ["actix-web", "fast web framework", "800000"],
            ["syn", "Parsing source code", "700000"],
            ["warp", "web server framework", "600000"],
            ["Ratatui", "terminal user interfaces", "500000"],
        ]
        .iter()
        .map(|row| Row::new(row.iter().map(|s| String::from(*s)).collect_vec()))
        .collect_vec();

        Table::new(rows, widths).header(Row::new(vec![
            "Name",
            "Description",
            "Downloads",
        ]))
    }
    // ANCHOR_END: app_results_table_widget

    // ANCHOR: app_prompt_widget
    fn prompt(&self) -> (Block, Paragraph) {
        let color = if matches!(self.mode, Mode::Prompt) {
            Color::Yellow
        } else {
            Color::Blue
        };
        let block = Block::default().borders(Borders::ALL).border_style(color);
        let paragraph = Paragraph::new("prompt");
        (block, paragraph)
    }
    // ANCHOR_END: app_prompt_widget
}

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
