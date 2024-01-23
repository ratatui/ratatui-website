use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame) {
    let tabs = Tabs::new(vec!["TODO", "IN PROGRESS", "DONE"])
        .block(Block::default().title("Project").borders(Borders::ALL))
        .style(Style::default().white())
        .highlight_style(Style::default().underlined().bold().yellow())
        .select(1)
        .divider(symbols::DOT)
        .padding(" ", " ");

    frame.render_widget(tabs, frame.size());
}
