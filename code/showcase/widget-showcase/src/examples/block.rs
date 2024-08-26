use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

pub fn render(frame: &mut Frame) {
    // intentionally mismatched border types to show how they look
    let border_set = symbols::border::Set {
        top_left: symbols::line::ROUNDED.top_left,
        top_right: symbols::line::THICK.top_right,
        bottom_left: symbols::line::ROUNDED.bottom_left,
        bottom_right: symbols::border::THICK.bottom_right,
        vertical_left: symbols::line::ROUNDED.vertical,
        vertical_right: symbols::line::THICK.vertical,
        horizontal_top: symbols::line::NORMAL.horizontal,
        horizontal_bottom: symbols::line::DOUBLE.horizontal,
    };
    let block = Block::default()
        .title("Left Title".yellow())
        .title(Title::from("Center title".blue()).alignment(Alignment::Center))
        .title(Title::from("Right Title".green()).alignment(Alignment::Right))
        .title(
            Title::from("Bottom Center title".blue())
                .alignment(Alignment::Center)
                .position(block::Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border_set)
        .border_style(Style::default().fg(Color::Red));
    frame.render_widget(
        Paragraph::new("A Block widget that wraps other widgets.".italic())
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        frame.area(),
    );
}
