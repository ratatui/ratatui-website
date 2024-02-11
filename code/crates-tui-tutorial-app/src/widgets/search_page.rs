use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Layout, Position},
    widgets::StatefulWidget,
};
use tokio::sync::mpsc::UnboundedSender;
use tui_input::backend::crossterm::EventHandler;

use crate::{
    app::{Action, Mode},
    crates_io_api_helper,
    widgets::{search_prompt::SearchPrompt, search_results::SearchResults},
};

use super::{
    search_prompt::SearchPromptWidget, search_results::SearchResultsWidget,
};

// ANCHOR: search_page
#[derive(Debug)]
pub struct SearchPage {
    pub results: SearchResults,
    pub prompt: SearchPrompt,

    pub page: u64,
    pub page_size: u64,
    pub sort: crates_io_api::Sort,
    pub crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
    pub loading_status: Arc<AtomicBool>,
    tx: UnboundedSender<Action>,
}
// ANCHOR_END: search_page

impl SearchPage {
    pub fn new(tx: UnboundedSender<Action>) -> Self {
        let loading_status = Arc::new(AtomicBool::default());
        Self {
            results: SearchResults::default(),
            prompt: SearchPrompt::default(),
            page: 1,
            page_size: 25,
            sort: crates_io_api::Sort::Relevance,
            crates: Default::default(),
            tx,
            loading_status,
        }
    }

    pub fn scroll_up(&mut self) {
        self.results.scroll_previous();
    }

    pub fn scroll_down(&mut self) {
        self.results.scroll_next();
    }

    pub fn loading(&self) -> bool {
        self.loading_status.load(Ordering::SeqCst)
    }

    // ANCHOR: prompt_methods
    pub fn handle_key(&mut self, key: KeyEvent) {
        self.prompt.input.handle_event(&CrosstermEvent::Key(key));
    }

    pub fn cursor_position(&self) -> Option<Position> {
        self.prompt.cursor_position
    }
    // ANCHOR_END: prompt_methods

    pub fn update_search_results(&mut self) {
        self.results.select(None);
        let crates: Vec<_> =
            self.crates.lock().unwrap().iter().cloned().collect_vec();
        self.results.crates = crates;
        self.results.content_length(self.results.crates.len());
        self.scroll_down();
    }

    // ANCHOR: submit
    pub fn submit_query(&mut self) {
        let _ = self.tx.send(Action::SwitchMode(Mode::Results));
        self.prepare_request();
        self.request_search_results();
    }

    pub fn prepare_request(&mut self) {
        self.results.select(None);
    }

    // ANCHOR: create_search_parameters
    pub fn create_search_parameters(
        &self,
    ) -> crates_io_api_helper::SearchParameters {
        crates_io_api_helper::SearchParameters::new(
            self.prompt.input.value().into(),
            self.crates.clone(),
            Some(self.tx.clone()),
        )
    }
    // ANCHOR_END: create_search_parameters

    // ANCHOR: request_search_results
    pub fn request_search_results(&self) {
        let loading_status = self.loading_status.clone();
        let mut params = self.create_search_parameters();
        params.fake_delay = 5;
        tokio::spawn(async move {
            loading_status.store(true, Ordering::SeqCst);
            let _ = crates_io_api_helper::request_search_results(&params).await;
            loading_status.store(false, Ordering::SeqCst);
        });
    }
    // ANCHOR_END: request_search_results

    // ANCHOR_END: submit
}

// ANCHOR: search_page_widget
pub struct SearchPageWidget {
    pub mode: Mode,
}

impl StatefulWidget for SearchPageWidget {
    type State = SearchPage;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let prompt_height = 5;

        let [main, prompt] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(prompt_height),
        ])
        .areas(area);

        SearchResultsWidget::new(matches!(self.mode, Mode::Results)).render(
            main,
            buf,
            &mut state.results,
        );

        SearchPromptWidget::new(self.mode, state.sort.clone()).render(
            prompt,
            buf,
            &mut state.prompt,
        );
    }
}
// ANCHOR_END: search_page_widget
