use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame) {
    let rows = [
        Row::new([
            "1".bold(),
            "joshka".into(),
            "john.doe@mail.org".dim(),
            "UTC -8".into(),
        ]),
        Row::new([
            "2".bold(),
            "kdheepak".into(),
            "joe.dane@mail.org".dim(),
            "UTC -5".into(),
        ]),
        Row::new([
            "3".bold(),
            "mindoodoo".into(),
            "jane.doe@mail.org".dim(),
            "UTC +7".into(),
        ]),
        Row::new([
            "4".bold(),
            "orhun".into(),
            "john.smith@mail.org".dim(),
            "UTC +3".into(),
        ]),
        Row::new([
            "5".bold(),
            "sayanarijit".into(),
            "john.smith@mail.org".dim(),
            "UTC +5:30".into(),
        ]),
    ];

    // width of 46
    let table = Table::new(
        rows,
        [
            Constraint::Length(2),
            Constraint::Length(11),
            Constraint::Length(19),
            Constraint::Length(9),
        ],
    )
    .header(
        Row::new(["id", "name", "mail", "timezone"])
            .bold()
            .underlined()
            .blue(),
    )
    .highlight_spacing(HighlightSpacing::Always)
    .highlight_style(Style::new().reversed().bold())
    .highlight_symbol("> ");

    let mut state = TableState::new().with_selected(Some(4));

    frame.render_stateful_widget(table, frame.size(), &mut state);
}
