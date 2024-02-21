use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use ratatui::prelude::*;
use ratatui::widgets::*;

#[derive(Debug)]
pub struct StatusBar {
    pub last_key_event: Option<crossterm::event::KeyEvent>,
    pub loading_status: Arc<AtomicBool>,
}

impl StatusBar {
    pub fn new(loading_status: Arc<AtomicBool>) -> Self {
        Self {
            last_key_event: None,
            loading_status,
        }
    }

    // ANCHOR: loading
    pub fn loading(&self) -> bool {
        self.loading_status.load(Ordering::SeqCst)
    }
}

// ANCHOR: app_widget
pub struct StatusBarWidget;
// ANCHOR_END: app_widget

// ANCHOR: app_statefulwidget
impl StatefulWidget for StatusBarWidget {
    type State = StatusBar;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let Some(key) = state.last_key_event {
            Paragraph::new(format!("last key event: {:?}", key.code))
                .right_aligned()
                .render(area, buf);
        }

        if state.loading() {
            Line::from("Loading...").render(area, buf);
        }
    }
}
// ANCHOR_END: app_statefulwidget
