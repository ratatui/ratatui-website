use ratatui::{prelude::*, widgets::Gauge};

pub fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.size());
    frame.render_widget(
        Gauge::default()
            .percent(50)
            .gauge_style(Style::new().light_red()),
        layout[0],
    );
    frame.render_widget(
        Gauge::default()
            .percent(50)
            .label("10/20")
            .gauge_style(Style::new().light_green().on_green()),
        layout[2],
    );
    frame.render_widget(
        Gauge::default()
            .percent(50)
            .gauge_style(Style::new().light_blue().on_blue())
            .style(Style::new().fg(Color::White).bg(Color::Blue)),
        layout[4],
    );
    frame.render_widget(
        Gauge::default()
            .ratio(0.51)
            .label("0.51")
            .use_unicode(true)
            .gauge_style(Style::new().white().on_dark_gray())
            .style(Style::new().fg(Color::White).bg(Color::Blue)),
        layout[6],
    );
}
