// ANCHOR: imports_all
// ANCHOR: imports_external
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::Paragraph};
// ANCHOR: imports_external

// ANCHOR: imports_core
use crate::{
    events::{Event, Events},
    tui::Tui,
    widgets::{search_page::SearchPage, search_page::SearchPageWidget},
};
// ANCHOR_END: imports_core
// ANCHOR_END: imports_all

// ANCHOR: full_app
// ANCHOR: mode
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    #[default]
    Prompt,
    Results,
}
// ANCHOR_END: mode

// ANCHOR: mode_handle_key
impl Mode {
    fn handle_key(&self, key: KeyEvent) -> Option<Action> {
        use crossterm::event::KeyCode::*;
        let action = match self {
            Mode::Prompt => match key.code {
                Enter => Action::SubmitSearchQuery,
                Esc => Action::SwitchMode(Mode::Results),
                _ => return None,
            },
            Mode::Results => match key.code {
                Up => Action::ScrollUp,
                Down => Action::ScrollDown,
                Char('/') => Action::SwitchMode(Mode::Prompt),
                Esc => Action::Quit,
                _ => return None,
            },
        };
        Some(action)
    }
}
// ANCHOR_END: mode_handle_key

// ANCHOR: action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    SwitchMode(Mode),
    ScrollDown,
    ScrollUp,
    SubmitSearchQuery,
    UpdateSearchResults,
}
// ANCHOR_END: action

// ANCHOR: app
#[derive(Debug)]
pub struct App {
    quit: bool,
    last_key_event: Option<crossterm::event::KeyEvent>,
    mode: Mode,
    rx: tokio::sync::mpsc::UnboundedReceiver<Action>,
    tx: tokio::sync::mpsc::UnboundedSender<Action>,

    search_page: SearchPage, // new
}
// ANCHOR_END: app

impl App {
    // ANCHOR: app_new
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let search_page = SearchPage::new(tx.clone());
        let mode = Mode::default();
        let quit = false;
        let last_key_event = None;
        Self {
            quit,
            last_key_event,
            mode,
            rx,
            tx,
            search_page,
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
                if matches!(e, Event::Render) {
                    self.draw(&mut tui)?
                } else {
                    self.handle_event(e)?
                }
            }
            while let Ok(action) = self.rx.try_recv() {
                self.handle_action(action)?;
            }
            if self.should_quit() {
                break;
            }
        }
        Ok(())
    }
    // ANCHOR_END: app_run

    // ANCHOR: app_handle_event
    fn handle_event(&mut self, e: Event) -> Result<()> {
        use crossterm::event::Event as CrosstermEvent;
        if let Event::Crossterm(CrosstermEvent::Key(key)) = e {
            self.last_key_event = Some(key);
            self.handle_key(key)
        };
        Ok(())
    }
    // ANCHOR_END: app_handle_event

    // ANCHOR: app_handle_key_event
    fn handle_key(&mut self, key: KeyEvent) {
        let maybe_action = self.mode.handle_key(key);
        if maybe_action.is_none() && matches!(self.mode, Mode::Prompt) {
            self.search_page.handle_key(key);
        }
        maybe_action.map(|action| self.tx.send(action));
    }
    // ANCHOR_END: app_handle_key_event

    // ANCHOR: app_handle_action
    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Quit => self.quit(),
            Action::SwitchMode(mode) => self.switch_mode(mode),
            Action::ScrollUp => self.search_page.scroll_up(),
            Action::ScrollDown => self.search_page.scroll_down(),
            Action::SubmitSearchQuery => self.search_page.submit_query(),
            Action::UpdateSearchResults => {
                self.search_page.update_search_results()
            }
        }
        Ok(())
    }
    // ANCHOR_END: app_handle_action
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    // ANCHOR: app_draw
    fn draw(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            frame.render_stateful_widget(AppWidget, frame.size(), self);
            self.set_cursor(frame);
        })?;
        Ok(())
    }
    // ANCHOR_END: app_draw

    fn quit(&mut self) {
        self.quit = true
    }

    fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn should_quit(&self) -> bool {
        self.quit
    }

    fn set_cursor(&mut self, frame: &mut Frame<'_>) {
        if matches!(self.mode, Mode::Prompt) {
            if let Some(cursor_position) = self.search_page.cursor_position() {
                frame.set_cursor(cursor_position.x, cursor_position.y)
            }
        }
    }
}

// ANCHOR: app_widget
struct AppWidget;
// ANCHOR_END: app_widget

// ANCHOR: app_statefulwidget
impl StatefulWidget for AppWidget {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [status_bar, search_page] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(0)])
                .areas(area);

        if let Some(key) = state.last_key_event {
            Paragraph::new(format!("last key event: {:?}", key.code))
                .right_aligned()
                .render(status_bar, buf);
        }

        if state.search_page.loading() {
            Line::from("Loading...").render(status_bar, buf);
        }

        SearchPageWidget { mode: state.mode }.render(
            search_page,
            buf,
            &mut state.search_page,
        );
    }
}
// ANCHOR_END: app_statefulwidget

// ANCHOR_END: full_app
