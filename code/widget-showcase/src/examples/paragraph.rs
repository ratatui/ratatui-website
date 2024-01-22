use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame) {
    let text = "Hello, Ratatui!\nThis is a colorful paragraph";

    let paragraph = Paragraph::new(text).bold().white().on_cyan();

    frame.render_widget(paragraph, frame.size());
}
