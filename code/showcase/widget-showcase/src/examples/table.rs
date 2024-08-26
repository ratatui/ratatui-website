use ratatui::{prelude::*, widgets::*};

struct Item {
    user: &'static str,
    mail: &'static str,
    timezone: &'static str,
}

impl Item {}

const ITEMS: [Item; 6] = [
    Item {
        user: "fdehau",
        mail: "Florian Dehau <fdehau@users.noreply.github.com",
        timezone: "UTC+1",
    },
    Item {
        user: "joshka",
        mail: "Josh McKinney <joshka@users.noreply.github.com>",
        timezone: "UTC-8",
    },
    Item {
        user: "kdheepak",
        mail: "Dheepak Krishnamurthy <me@kdheepak.com>",
        timezone: "UTC-5",
    },
    Item {
        user: "mindoodoo",
        mail: "Leon Sautour <minindoo@users.noreply.github.com>",
        timezone: "UTC+1",
    },
    Item {
        user: "orhun",
        mail: "Orhun Parmaksız <orhun@archlinux.org>",
        timezone: "UTC+3",
    },
    Item {
        user: "Valentin271",
        mail: "Valentin271 <36198422+Valentin271@users.noreply.github.com>",
        timezone: "UTC+1",
    },
];

pub fn render(frame: &mut Frame) {
    let rows = ITEMS.iter().map(|item| {
        Row::new([
            item.user.into(),
            item.mail.dim().into(),
            Line::from(item.timezone).alignment(Alignment::Right),
        ])
    });
    // width of 46
    let table = Table::new(
        rows,
        [
            Constraint::Length(11),
            Constraint::Length(29),
            Constraint::Length(5),
        ],
    )
    .header(Row::new(["User", "Email", "TZ"]).bold().underlined().blue())
    .highlight_spacing(HighlightSpacing::Always)
    .highlight_style(Style::new().reversed().bold())
    .highlight_symbol(">");

    let mut state = TableState::new().with_selected(Some(4));

    frame.render_stateful_widget(table, frame.area(), &mut state);
}
