use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame) {
    let sparkline = Sparkline::default()
        .block(Block::default().title("Sparkline").borders(Borders::ALL))
        .data(&[
            35, 40, 44, 49, 52, 55, 58, 59, 60, 60, 59, 57, 55, 52, 48, 44, 39, 34, 29, 24, 19, 15,
            11, 7, 4, 2, 1, 0, 0, 1, 3, 6, 9, 13, 17, 22, 27, 32, 36, 41, 46, 50, 53, 56, 58, 59,
            60, 60, 59, 57, 54, 51, 47, 42, 38, 33, 28, 23, 18,
        ])
        .max(62)
        .direction(RenderDirection::LeftToRight)
        .style(Style::default().yellow());

    frame.render_widget(sparkline, frame.area());
}
