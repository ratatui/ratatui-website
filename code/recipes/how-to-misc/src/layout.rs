#![allow(dead_code)]
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    text::Text,
    widgets::{Block, Clear, Paragraph},
    Frame,
};

// Examples for https://ratatui.rs/recipes/layout/center-a-rect/

// ANCHOR: center
/// Centers a [`Rect`] within another [`Rect`] using the provided [`Constraint`]s.
///
/// # Examples
///
/// ```rust
/// use ratatui::layout::{Constraint, Rect};
///
/// let area = Rect::new(0, 0, 100, 100);
/// let horizontal = Constraint::Percentage(20);
/// let vertical = Constraint::Percentage(30);
///
/// let centered = center(area, horizontal, vertical);
/// ```
fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
// ANCHOR_END: center

// ANCHOR: render
fn render(frame: &mut Frame) {
    let text = Text::raw("Hello world!");
    let area = center(
        frame.area(),
        Constraint::Length(text.width() as u16),
        Constraint::Length(1),
    );
    frame.render_widget(text, area);
}
// ANCHOR_END: render

// ANCHOR: render_popup
fn render_popup(frame: &mut Frame) {
    let area = center(
        frame.area(),
        Constraint::Percentage(20),
        Constraint::Length(3), // top and bottom border + content
    );
    let popup = Paragraph::new("Popup content").block(Block::bordered().title("Popup"));
    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}
// ANCHOR_END: render_popup

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center() {
        let area = Rect::new(0, 0, 100, 100);
        let horizontal = Constraint::Percentage(20);
        let vertical = Constraint::Percentage(30);

        assert_eq!(
            center(area, horizontal, vertical),
            Rect {
                x: 40,
                y: 35,
                width: 20,
                height: 30,
            }
        );
    }
}
