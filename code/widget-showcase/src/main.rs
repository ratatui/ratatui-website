use std::{panic, time::Duration};

use clap::{Parser, ValueEnum};
use color_eyre::{
    config::HookBuilder,
    eyre::{self, WrapErr},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        block::Title,
        calendar::{CalendarEventStore, Monthly},
        *,
    },
};
use time::{Date, Month};
mod tui;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The widget to display.
    #[arg(short, long)]
    widget: Widget,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Widget {
    Block,
    BarChart,
    Calendar,
}

fn main() -> color_eyre::Result<()> {
    install_hooks()?;
    let args = Args::parse();
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal, args.widget)?;
    tui::restore()?;
    Ok(())
}

/// This replaces the standard color_eyre panic and error hooks with hooks that
/// restore the terminal before printing the panic or error.
pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();

    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        tui::restore().unwrap();
        panic_hook(panic_info);
    }));

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            tui::restore().unwrap();
            eyre_hook(error)
        },
    ))?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct App {
    running_state: RunningState,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Finished,
}
impl App {
    fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        widget: Widget,
    ) -> color_eyre::Result<()> {
        while self.running_state != RunningState::Finished {
            terminal.draw(|frame| self.render_frame(frame, widget).unwrap())?;
            self.update().wrap_err("update failed")?;
        }
        Ok(())
    }
    fn render_frame(
        &self,
        frame: &mut Frame,
        widget: Widget,
    ) -> color_eyre::Result<()> {
        match widget {
            Widget::Block => render_block(frame),
            Widget::BarChart => render_bar_chart(frame),
            Widget::Calendar => render_calendar(frame)?,
        }
        Ok(())
    }

    fn update(&mut self) -> color_eyre::Result<()> {
        // quit if a timeout occurs
        if !event::poll(Duration::from_secs(3))? {
            self.running_state = RunningState::Finished;
        }
        match event::read()? {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.running_state = RunningState::Finished;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}

fn render_block(frame: &mut Frame) {
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
        frame.size(),
    );
}

#[allow(dead_code)]
fn title_block<'a>(_title: &'a str) -> Block<'a> {
    // Block::default()
    //     .title(Title::from(title).alignment(Alignment::Center))
    //     .borders(Borders::ALL)
    //     .border_set(symbols::border::THICK)
    //     .border_style(Style::new().dark_gray())
    Block::default()
}

fn render_bar_chart(frame: &mut Frame) {
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
        .data(data.clone())
        .block(title_block("Bar Chart"));
    let horizontal = BarChart::default()
        .bar_width(1)
        .bar_gap(1)
        .data(data)
        .direction(Direction::Horizontal)
        .block(title_block("Horizontal Bar Chart"));
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(frame.size());
    frame.render_widget(vertical, layout[0]);
    frame.render_widget(horizontal, layout[1]);
}

fn render_calendar(frame: &mut Frame) -> color_eyre::Result<()> {
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
    let washingtons_birthday =
        Date::from_calendar_date(2023, Month::February, 20)?;
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
