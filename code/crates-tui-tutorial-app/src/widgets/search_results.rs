use std::sync::{Arc, Mutex};

use crates_io_api::Crate;
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

// ANCHOR: state
#[derive(Debug, Default)]
pub struct SearchResults {
    pub crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
    pub table_state: TableState,
    pub scrollbar_state: ScrollbarState,
}

impl SearchResults {
    pub fn new(crates: Arc<Mutex<Vec<crates_io_api::Crate>>>) -> Self {
        Self {
            crates,
            table_state: Default::default(),
            scrollbar_state: Default::default(),
        }
    }
}
// ANCHOR_END: state

impl SearchResults {
    fn rows(&self) -> Vec<Row<'static>> {
        self.crates
            .lock()
            .unwrap()
            .iter()
            .map(row_from_crate)
            .collect_vec()
    }

    fn header(&self) -> Row<'static> {
        let header_cells = ["Name", "Description", "Downloads"]
            .map(|h| h.bold().into())
            .map(vertical_pad);
        Row::new(header_cells).height(TABLE_HEADER_HEIGHT)
    }

    pub fn clear_selection(&mut self) {
        self.table_state.select(None)
    }

    // ANCHOR: scroll
    pub fn scroll_next(&mut self) {
        let wrap_index = self.crates.lock().unwrap().len().max(1);
        let next = self
            .table_state
            .selected()
            .map_or(0, |i| (i + 1) % wrap_index);
        self.scroll_to(next);
    }

    pub fn scroll_previous(&mut self) {
        let last = self.crates.lock().unwrap().len().saturating_sub(1);
        let wrap_index = self.crates.lock().unwrap().len().max(1);
        let previous = self
            .table_state
            .selected()
            .map_or(last, |i| (i + last) % wrap_index);
        self.scroll_to(previous);
    }

    fn scroll_to(&mut self, index: usize) {
        if self.crates.lock().unwrap().is_empty() {
            self.table_state.select(None)
        } else {
            self.table_state.select(Some(index));
            self.scrollbar_state = self.scrollbar_state.position(index);
        }
    }

    pub fn update_search_results(&mut self) {
        self.table_state.select(None);
        self.scrollbar_state = self
            .scrollbar_state
            .content_length(self.crates.lock().unwrap().len())
    }

    // ANCHOR_END: scroll
}

const TABLE_HEADER_HEIGHT: u16 = 2;
const COLUMN_SPACING: u16 = 3;
const ROW_HEIGHT: u16 = 2;

// ANCHOR: widget
pub struct SearchResultsWidget {
    highlight: bool,
}

impl SearchResultsWidget {
    pub fn new(highlight: bool) -> Self {
        Self { highlight }
    }
}
// ANCHOR_END: widget

impl SearchResultsWidget {
    fn render_scrollbar(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut SearchResults,
    ) {
        let [_, scrollbar_area] = Layout::vertical([
            Constraint::Length(TABLE_HEADER_HEIGHT),
            Constraint::Fill(1),
        ])
        .areas(area);

        Scrollbar::default()
            .track_symbol(Some(" "))
            .thumb_symbol("▐")
            .begin_symbol(None)
            .end_symbol(None)
            .render(scrollbar_area, buf, &mut state.scrollbar_state);
    }

    fn render_table(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut SearchResults,
    ) {
        let highlight_symbol = if self.highlight {
            " █ "
        } else {
            " \u{2022} "
        };

        let column_widths = [
            Constraint::Max(20),
            Constraint::Fill(1),
            Constraint::Max(11),
        ];

        let header = state.header();
        let rows = state.rows();

        let table = Table::new(rows, column_widths)
            .header(header)
            .column_spacing(COLUMN_SPACING)
            .highlight_symbol(vertical_pad(highlight_symbol.into()))
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(table, area, buf, &mut state.table_state);
    }
}

// ANCHOR: render
impl StatefulWidget for SearchResultsWidget {
    type State = SearchResults;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [table_area, scrollbar_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)])
                .areas(area);

        self.render_scrollbar(scrollbar_area, buf, state);
        self.render_table(table_area, buf, state);
    }
}
// ANCHOR_END: render

fn vertical_pad(line: Line) -> Text {
    Text::from(vec!["".into(), line])
}

fn row_from_crate(krate: &Crate) -> Row<'static> {
    let crate_name = Line::from(krate.name.clone());
    let description = Line::from(krate.description.clone().unwrap_or_default());
    let downloads = Line::from(krate.downloads.to_string()).right_aligned();
    Row::new([
        vertical_pad(crate_name),
        vertical_pad(description),
        vertical_pad(downloads),
    ])
    .height(ROW_HEIGHT)
}
