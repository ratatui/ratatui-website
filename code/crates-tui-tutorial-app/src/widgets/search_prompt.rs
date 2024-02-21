use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use ratatui::{layout::Position, prelude::*, widgets::*};
use tui_input::backend::crossterm::EventHandler;

use crate::app::{Action, Mode};

// ANCHOR: state
#[derive(Debug, Clone)]
pub struct SearchPrompt {
    pub cursor_position: Option<Position>,
    pub input: tui_input::Input,
    sort: crates_io_api::Sort,
    tx: tokio::sync::mpsc::UnboundedSender<Action>,
    loading_status: Arc<AtomicBool>,
    crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
}

impl SearchPrompt {
    pub fn new(
        tx: tokio::sync::mpsc::UnboundedSender<Action>,
        loading_status: Arc<AtomicBool>,
        crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
    ) -> Self {
        Self {
            cursor_position: Default::default(),
            input: Default::default(),
            sort: crates_io_api::Sort::Relevance,
            tx,
            loading_status,
            crates,
        }
    }

    // ANCHOR: prompt_methods
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::Event as CrosstermEvent;
        self.input.handle_event(&CrosstermEvent::Key(key));
    }

    // ANCHOR: submit
    pub fn submit_query(&mut self) {
        let tx = self.tx.clone();
        let loading_status = self.loading_status.clone();
        let params = self.create_search_parameters();
        // params.fake_delay = 5;
        tokio::spawn(async move {
            loading_status.store(true, Ordering::SeqCst);
            let _ =
                crate::crates_io_api_helper::request_search_results(&params)
                    .await;
            loading_status.store(false, Ordering::SeqCst);
            let _ = tx.send(Action::UpdateSearchResults);
            let _ = tx.send(Action::SwitchMode(Mode::Results));
            let _ = tx.send(Action::ScrollDown);
        });
    }

    // ANCHOR: create_search_parameters
    pub fn create_search_parameters(
        &self,
    ) -> crate::crates_io_api_helper::SearchParameters {
        crate::crates_io_api_helper::SearchParameters::new(
            self.input.value().into(),
            self.crates.clone(),
        )
    }
    // ANCHOR_END: create_search_parameters

    // ANCHOR_END: submit
}

// ANCHOR_END: state

// ANCHOR: widget
pub struct SearchPromptWidget {
    focused: bool,
}
// ANCHOR_END: widget

impl SearchPromptWidget {
    pub fn new(focused: bool) -> Self {
        Self { focused }
    }

    fn border(&self) -> Block {
        let color = if self.focused {
            Color::Yellow
        } else {
            Color::Black
        };
        Block::default().borders(Borders::ALL).border_style(color)
    }

    fn sort_by_text(&self, sort: crates_io_api::Sort) -> impl Widget {
        Paragraph::new(Line::from(vec![
            "Sort By: ".into(),
            format!("{:?}", sort).fg(Color::Blue),
        ]))
        .right_aligned()
    }

    fn prompt_text<'a>(
        &self,
        width: usize,
        input: &'a tui_input::Input,
    ) -> impl Widget + 'a {
        let scroll = input.cursor().saturating_sub(width.saturating_sub(4));
        let text = Line::from(vec![input.value().into()]);
        Paragraph::new(text).scroll((0, scroll as u16))
    }

    fn calculate_cursor_position(&self, area: Rect, state: &mut SearchPrompt) {
        if self.focused {
            let margin = (2, 2);
            let width = (area.width as f64 as u16).saturating_sub(margin.0);
            state.cursor_position = Some(Position::new(
                (area.x + margin.0 + state.input.cursor() as u16).min(width),
                area.y + margin.1,
            ));
        } else {
            state.cursor_position = None
        }
    }
}

// ANCHOR: render
impl StatefulWidget for SearchPromptWidget {
    type State = SearchPrompt;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.border().render(area, buf);

        let [input, meta] =
            Layout::horizontal([Constraint::Fill(0), Constraint::Length(25)])
                .areas(area);

        self.sort_by_text(state.sort.clone())
            .render(meta.inner(&Margin::new(2, 2)), buf);
        self.prompt_text(input.width as usize, &state.input)
            .render(input.inner(&Margin::new(2, 2)), buf);

        self.calculate_cursor_position(input, state);
    }
}
// ANCHOR_END: render
