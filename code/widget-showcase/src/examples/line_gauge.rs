use ratatui::{prelude::*, widgets::LineGauge};

pub fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.size());
    frame.render_widget(
        LineGauge::default()
            .ratio(0.5)
            .filled_style(Style::new().light_red().on_red()),
        layout[0],
    );
    frame.render_widget(
        LineGauge::default()
            .ratio(0.5)
            .label("0.5")
            .line_set(symbols::line::DOUBLE)
            .filled_style(Style::new().light_green().on_green()),
        layout[2],
    );
    frame.render_widget(
        LineGauge::default()
            .ratio(0.5)
            .label("10/20")
            .line_set(symbols::line::THICK)
            .filled_style(Style::new().light_blue().on_blue())
            .style(Style::new().blue()),
        layout[4],
    );
    frame.render_widget(
        LineGauge::default()
            .ratio(0.50)
            .filled_style(Style::new().white().on_black())
            .line_set(symbols::line::Set {
                horizontal: symbols::DOT,
                ..Default::default()
            })
            .style(Style::new().fg(Color::White).bg(Color::Blue)),
        layout[6],
    );
}
