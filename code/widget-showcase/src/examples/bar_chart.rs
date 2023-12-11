use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame) {
    let data = BarGroup::default().bars(&[
        Bar::default()
            .label("Red".into())
            .value(2)
            .style(Style::new().red()),
        Bar::default()
            .label("Green".into())
            .value(7)
            .style(Style::new().green()),
        Bar::default()
            .label("Blue".into())
            .value(11)
            .style(Style::new().blue()),
    ]);
    let vertical = BarChart::default()
        .bar_width(5)
        .bar_gap(1)
        .data(data.clone());
    let horizontal = BarChart::default()
        .bar_width(1)
        .bar_gap(1)
        .data(data)
        .direction(Direction::Horizontal);
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(frame.size());
    frame.render_widget(vertical, layout[0]);
    frame.render_widget(horizontal, layout[1]);
}
