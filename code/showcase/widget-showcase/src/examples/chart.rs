use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame) {
    let datasets = vec![
        Dataset::default()
            .name("data1")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&[
                (0.0, 10.0),
                (1.0, 7.7),
                (2.0, 2.9),
                (3.0, 0.1),
                (4.0, 1.7),
                (5.0, 6.4),
                (6.0, 9.8),
                (7.0, 8.8),
                (8.0, 4.3),
                (9.0, 0.4),
                (10.0, 0.8),
            ]),
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&[
                (0.0, 5.0),
                (1.0, 9.2),
                (2.0, 9.5),
                (3.0, 5.7),
                (4.0, 1.2),
                (5.0, 0.2),
                (6.0, 3.6),
                (7.0, 8.3),
                (8.0, 9.9),
                (9.0, 7.1),
                (10.0, 2.3),
            ]),
    ];
    let chart = Chart::new(datasets)
        .x_axis(
            Axis::default()
                .title(Span::styled("X", Style::default().fg(Color::Red)))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 10.0])
                .labels(["0.0", "5.0", "10.0"]),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled("Y", Style::default().fg(Color::Red)))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 10.0])
                .labels(["0.0", "10.0"]),
        );
    frame.render_widget(chart, frame.area());
}
