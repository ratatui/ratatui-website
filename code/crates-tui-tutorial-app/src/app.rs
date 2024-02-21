use std::sync::{atomic::AtomicBool, Arc, Mutex};

// ANCHOR: imports_all
// ANCHOR: imports_external
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
// ANCHOR: imports_external

// ANCHOR: imports_core
use crate::{
    events::{Event, Events},
    tui::Tui,
    widgets::{
        search_prompt::{SearchPrompt, SearchPromptWidget},
        search_results::{SearchResults, SearchResultsWidget},
        status_bar::{StatusBar, StatusBarWidget},
    },
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
    mode: Mode,
    rx: tokio::sync::mpsc::UnboundedReceiver<Action>,
    tx: tokio::sync::mpsc::UnboundedSender<Action>,

    status_bar: StatusBar,
    results: SearchResults,
    prompt: SearchPrompt,
}
// ANCHOR_END: app

impl App {
    // ANCHOR: app_new
    pub fn new() -> Self {
        let loading_status = Arc::new(AtomicBool::default());
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let crates: Arc<Mutex<Vec<crates_io_api::Crate>>> = Default::default();
        let results = SearchResults::new(crates.clone());
        let prompt = SearchPrompt::new(
            tx.clone(),
            loading_status.clone(),
            crates.clone(),
        );
        let status_bar = StatusBar::new(loading_status);
        let mode = Mode::default();
        let quit = false;
        Self {
            quit,
            mode,
            rx,
            tx,
            status_bar,
            results,
            prompt,
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
            self.status_bar.last_key_event = Some(key);
            self.handle_key(key)
        };
        Ok(())
    }
    // ANCHOR_END: app_handle_event

    // ANCHOR: app_handle_key_event
    fn handle_key(&mut self, key: KeyEvent) {
        let maybe_action = self.mode.handle_key(key);
        if maybe_action.is_none() && matches!(self.mode, Mode::Prompt) {
            self.prompt.handle_key(key);
        }
        maybe_action.map(|action| self.tx.send(action));
    }
    // ANCHOR_END: app_handle_key_event

    // ANCHOR: app_handle_action
    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Quit => self.quit(),
            Action::SwitchMode(mode) => self.switch_mode(mode),
            Action::ScrollUp => self.results.scroll_previous(),
            Action::ScrollDown => self.results.scroll_next(),
            Action::SubmitSearchQuery => {
                self.results.clear_selection();
                self.prompt.submit_query();
            }
            Action::UpdateSearchResults => self.results.update_search_results(),
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

    // ANCHOR: loading
    fn set_cursor(&mut self, frame: &mut Frame<'_>) {
        if matches!(self.mode, Mode::Prompt) {
            if let Some(cursor_position) = self.prompt.cursor_position {
                frame.set_cursor(cursor_position.x, cursor_position.y)
            }
        }
    }
}

const PROMPT_HEIGHT: u16 = 5;

// ANCHOR: app_widget
struct AppWidget;
// ANCHOR_END: app_widget

// ANCHOR: app_statefulwidget
impl StatefulWidget for AppWidget {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [status_bar, main, prompt] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(PROMPT_HEIGHT),
        ])
        .areas(area);

        StatusBarWidget.render(status_bar, buf, &mut state.status_bar);

        SearchResultsWidget::new(matches!(state.mode, Mode::Results)).render(
            main,
            buf,
            &mut state.results,
        );

        SearchPromptWidget::new(matches!(state.mode, Mode::Prompt)).render(
            prompt,
            buf,
            &mut state.prompt,
        );
    }
}
// ANCHOR_END: app_statefulwidget

// ANCHOR_END: full_app
