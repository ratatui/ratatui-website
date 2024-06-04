use ratatui::{prelude::*, widgets::calendar::*};
use time::{Date, Month};

pub fn render(frame: &mut Frame) -> color_eyre::Result<()> {
    let default_style = Style::new().black().on_gray();

    let january = Date::from_calendar_date(2023, Month::January, 1)?;
    let new_years_day = Date::from_calendar_date(2023, Month::January, 2)?;
    let mlk_day = Date::from_calendar_date(2023, Month::January, 16)?;
    let australia_day = Date::from_calendar_date(2023, Month::January, 26)?;
    let mut events = CalendarEventStore::default();
    events.add(new_years_day, Style::new().on_blue());
    events.add(mlk_day, Style::new().dark_gray().on_black());
    events.add(
        australia_day,
        Style::new().not_bold().green().on_light_yellow(),
    );

    let january_calendar = Monthly::new(january, events)
        .show_month_header(Style::default())
        .default_style(default_style);

    let february = Date::from_calendar_date(2023, Month::February, 1)?;
    let washingtons_birthday = Date::from_calendar_date(2023, Month::February, 20)?;
    let mut events = CalendarEventStore::default();
    events.add(
        washingtons_birthday,
        Style::new()
            .red()
            .on_white()
            .underline_color(Color::Blue)
            .underlined(),
    );
    let february_calendar = Monthly::new(february, events)
        .show_weekdays_header(Style::default())
        .show_surrounding(Style::new().dim())
        .default_style(default_style);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(23), Constraint::Min(0)])
        .split(frame.size());
    frame.render_widget(january_calendar, layout[0]);
    frame.render_widget(february_calendar, layout[1]);
    Ok(())
}
