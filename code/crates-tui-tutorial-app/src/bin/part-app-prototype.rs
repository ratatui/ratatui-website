use crates_tui::errors;
use crates_tui::events;
use crates_tui::tui;

use color_eyre::{eyre::Context, Result};
use events::{Event, Events};
use itertools::Itertools;
use ratatui::layout::Position;
use ratatui::prelude::*;
use ratatui::widgets::*;
use tui::Tui;

// ANCHOR: helper
use crates_io_api::CratesQuery;
use std::sync::{Arc, Mutex};
use tui_input::backend::crossterm::EventHandler;

// ANCHOR: search_parameters
/// Represents the parameters needed for fetching crates asynchronously.
pub struct SearchParameters {
    // Request
    pub search: String,
    pub page: u64,
    pub page_size: u64,
    pub sort: crates_io_api::Sort,

    // Response
    pub crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
}
// ANCHOR_END: search_parameters

impl SearchParameters {
    pub fn new(
        search: String,
        crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
    ) -> SearchParameters {
        SearchParameters {
            search,
            page: 1,
            page_size: 100,
            sort: crates_io_api::Sort::Relevance,
            crates,
        }
    }
}

// ANCHOR: request_search_results
/// Performs the actual search, and sends the result back through the
/// sender.
pub async fn request_search_results(
    params: &SearchParameters,
) -> Result<(), String> {
    let client = create_client()?;
    let query = create_query(params);
    let crates = fetch_crates_and_metadata(client, query).await?;
    update_state_with_fetched_crates(crates, params);
    Ok(())
}
// ANCHOR_END: request_search_results

/// Helper function to create client and fetch crates, wrapping both actions
/// into a result pattern.
fn create_client() -> Result<crates_io_api::AsyncClient, String> {
    // ANCHOR: client
    let email = std::env::var("CRATES_TUI_TUTORIAL_APP_MYEMAIL").context("Need to set CRATES_TUI_TUTORIAL_APP_MYEMAIL environment variable to proceed").unwrap();

    let user_agent = format!("crates-tui ({email})");
    let rate_limit = std::time::Duration::from_millis(1000);

    crates_io_api::AsyncClient::new(&user_agent, rate_limit)
        // ANCHOR_END: client
        .map_err(|err| format!("API Client Error: {err:#?}"))
}

// ANCHOR: create_query
fn create_query(params: &SearchParameters) -> CratesQuery {
    crates_io_api::CratesQueryBuilder::default()
        .search(&params.search)
        .page(params.page)
        .page_size(params.page_size)
        .sort(params.sort.clone())
        .build()
}
// ANCHOR_END: create_query

async fn fetch_crates_and_metadata(
    client: crates_io_api::AsyncClient,
    query: crates_io_api::CratesQuery,
) -> Result<Vec<crates_io_api::Crate>, String> {
    // ANCHOR: crates_query
    let page_result = client
        .crates(query)
        .await
        // ANCHOR_END: crates_query
        .map_err(|err| format!("API Client Error: {err:#?}"))?;
    let crates = page_result.crates;
    Ok(crates)
}

/// Handles the result after fetching crates and sending corresponding
/// actions.
fn update_state_with_fetched_crates(
    crates: Vec<crates_io_api::Crate>,
    params: &SearchParameters,
) {
    // ANCHOR: update_state
    let mut app_crates = params.crates.lock().unwrap();
    app_crates.clear();
    app_crates.extend(crates);
    // ANCHOR_END: update_state
}
// ANCHOR_END: helper

// ANCHOR: full_app

// ANCHOR: mode
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    #[default]
    Prompt,
    Results,
}

// ANCHOR: mode_handle_key
impl Mode {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Action> {
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
// ANCHOR_END: mode

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
pub struct App {
    quit: bool,
    last_key_event: Option<crossterm::event::KeyEvent>,
    mode: Mode,
    crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
    table_state: TableState,
    prompt: tui_input::Input,
    cursor_position: Option<Position>,

    tx: tokio::sync::mpsc::UnboundedSender<Action>, // new
    rx: tokio::sync::mpsc::UnboundedReceiver<Action>, // new
}
// ANCHOR_END: app

impl App {
    // ANCHOR: app_new
    pub fn new() -> Self {
        let quit = false;
        let mode = Mode::default();
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let crates = Default::default();
        let table_state = TableState::default();
        let prompt = Default::default();
        let cursor_position = None;
        let last_key_event = None;
        Self {
            quit,
            mode,
            last_key_event,
            tx,
            rx,
            crates,
            table_state,
            prompt,
            cursor_position,
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
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::Event as CrosstermEvent;
        let maybe_action = self.mode.handle_key(key);
        if maybe_action.is_none() && matches!(self.mode, Mode::Prompt) {
            self.prompt.handle_event(&CrosstermEvent::Key(key));
        }
        maybe_action.map(|action| self.tx.send(action));
    }
    // ANCHOR_END: app_handle_key_event

    // ANCHOR: app_handle_action
    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Quit => self.quit(),
            Action::SwitchMode(mode) => self.switch_mode(mode),
            Action::ScrollUp => self.scroll_up(),
            Action::ScrollDown => self.scroll_down(),
            Action::SubmitSearchQuery => self.submit_search_query(),
            Action::UpdateSearchResults => self.update_search_results(),
        }
        Ok(())
    }
    // ANCHOR_END: app_handle_action

    // ANCHOR: app_draw
    fn draw(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            frame.render_stateful_widget(AppWidget, frame.size(), self);
            self.set_cursor(frame);
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

    fn scroll_up(&mut self) {
        let last = self.crates.lock().unwrap().len().saturating_sub(1);
        let wrap_index = self.crates.lock().unwrap().len().max(1);
        let previous = self
            .table_state
            .selected()
            .map_or(last, |i| (i + last) % wrap_index);
        self.scroll_to(previous);
    }

    fn scroll_down(&mut self) {
        let wrap_index = self.crates.lock().unwrap().len().max(1);
        let next = self
            .table_state
            .selected()
            .map_or(0, |i| (i + 1) % wrap_index);
        self.scroll_to(next);
    }

    fn scroll_to(&mut self, index: usize) {
        if self.crates.lock().unwrap().is_empty() {
            self.table_state.select(None)
        } else {
            self.table_state.select(Some(index));
        }
    }

    // ANCHOR: app_submit_search_query
    fn submit_search_query(&mut self) {
        // prepare request
        self.table_state.select(None);
        let params = SearchParameters::new(
            self.prompt.value().into(),
            self.crates.clone(),
        );
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let _ = request_search_results(&params).await;
            tx.send(Action::UpdateSearchResults) // new
        });
        self.switch_mode(Mode::Results);
    }
    // ANCHOR_END: app_submit_search_query

    // ANCHOR: app_update_search_results
    fn update_search_results(&mut self) {
        self.scroll_down(); // select first item in the results if they exist
    }
    // ANCHOR_END: app_update_search_results

    fn set_cursor(&mut self, frame: &mut Frame<'_>) {
        if matches!(self.mode, Mode::Prompt) {
            if let Some(cursor_position) = self.cursor_position {
                frame.set_cursor(cursor_position.x, cursor_position.y)
            }
        }
    }
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
        StatefulWidget::render(table, results, buf, &mut state.table_state);

        let (block, paragraph) = state.prompt();
        block.render(prompt, buf);
        paragraph.render(
            prompt.inner(&Margin {
                horizontal: 2,
                vertical: 2,
            }),
            buf,
        );
        state.calculate_cursor_position(prompt);

        if let Some(key) = state.last_key_event {
            Paragraph::new(format!("last key event: {:?}", key.code))
                .right_aligned()
                .render(last_key_event, buf);
        }
    }
}

impl App {
    fn results(&self) -> Table<'static> {
        let widths = [
            Constraint::Length(15),
            Constraint::Min(0),
            Constraint::Length(20),
        ];

        let crates = self.crates.lock().unwrap();

        let rows = crates
            .iter()
            .map(|krate| {
                vec![
                    krate.name.clone(),
                    krate.description.clone().unwrap_or_default(),
                    krate.downloads.to_string(),
                ]
            })
            .map(|row| Row::new(row.iter().map(String::from).collect_vec()))
            .collect_vec();

        Table::new(rows, widths)
            .header(Row::new(vec!["Name", "Description", "Downloads"]))
            .highlight_symbol(" █ ")
            .highlight_spacing(HighlightSpacing::Always)
    }

    fn prompt(&self) -> (Block, Paragraph) {
        let color = if matches!(self.mode, Mode::Prompt) {
            Color::Yellow
        } else {
            Color::Blue
        };
        let block = Block::default().borders(Borders::ALL).border_style(color);

        let paragraph = Paragraph::new(self.prompt.value());

        (block, paragraph)
    }

    fn calculate_cursor_position(&mut self, area: Rect) {
        if matches!(self.mode, Mode::Prompt) {
            let margin = (2, 2);
            let width = (area.width as f64 as u16).saturating_sub(margin.0);
            self.cursor_position = Some(Position::new(
                (area.x + margin.0 + self.prompt.cursor() as u16).min(width),
                area.y + margin.1,
            ));
        } else {
            self.cursor_position = None
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
