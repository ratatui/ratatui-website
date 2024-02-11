use ratatui::{layout::Position, prelude::*, widgets::*};

use crate::app::Mode;

// ANCHOR: state
#[derive(Default, Debug, Clone)]
pub struct SearchPrompt {
    pub cursor_position: Option<Position>,
    pub input: tui_input::Input,
}
// ANCHOR_END: state

// ANCHOR: widget
pub struct SearchPromptWidget {
    mode: Mode,
    sort: crates_io_api::Sort,
}
// ANCHOR_END: widget

impl SearchPromptWidget {
    pub fn new(mode: Mode, sort: crates_io_api::Sort) -> Self {
        Self { mode, sort }
    }

    fn border(&self) -> Block {
        let color = if matches!(self.mode, Mode::Prompt) {
            Color::Yellow
        } else {
            Color::Black
        };
        Block::default().borders(Borders::ALL).border_style(color)
    }

    fn sort_by_text(&self) -> impl Widget {
        Paragraph::new(Line::from(vec![
            "Sort By: ".into(),
            format!("{:?}", self.sort.clone()).fg(Color::Blue),
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
        if matches!(self.mode, Mode::Prompt) {
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

        self.sort_by_text()
            .render(meta.inner(&Margin::new(2, 2)), buf);
        self.prompt_text(input.width as usize, &state.input)
            .render(input.inner(&Margin::new(2, 2)), buf);

        self.calculate_cursor_position(input, state);
    }
}
// ANCHOR_END: render
