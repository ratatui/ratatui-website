use ratatui::{
    prelude::*,
    widgets::{HighlightSpacing, List, ListState},
};

pub fn render(frame: &mut Frame) {
    let list = List::new([
        Line::from(vec!["✔️ ".green(), "Fork tui-rs 💻".into()]),
        Line::from(vec![
            "✔️ ".green(),
            "Create a ".into(),
            "great".italic(),
            " mascot 🐀".into(),
        ]),
        Line::from(vec!["✔️ ".green(), "Create a website & book 🕮".into()]),
        Line::from(vec!["✔️ ".green(), "Celebrate 500th commit ⭐".into()]),
        Line::from(vec!["✔️ ".green(), "Celebrate 1000th commit ✨".into()]),
        Line::from(vec!["⌛".yellow(), "Release Ratatui 1.0.0 🎉".bold()]),
    ])
    .highlight_symbol("» ")
    .highlight_spacing(HighlightSpacing::Always);

    let mut state = ListState::default().with_selected(Some(5));

    frame.render_stateful_widget(list, frame.size(), &mut state);
}
