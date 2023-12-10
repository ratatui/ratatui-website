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
use time::Date;
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
            terminal.draw(|frame| self.render_frame(frame, widget))?;
            self.update().wrap_err("update failed")?;
        }
        Ok(())
    }
    fn render_frame(&self, frame: &mut Frame, widget: Widget) {
        match widget {
            Widget::Block => render_block(frame),
            Widget::BarChart => render_bar_chart(frame),
            Widget::Calendar => render_calendar(frame),
        }
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

fn title_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .title(Title::from(title).alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_set(symbols::border::THICK)
        .border_style(Style::new().dark_gray())
}

fn render_bar_chart(frame: &mut Frame) {
    let barchart = BarChart::default()
        .bar_width(3)
        .bar_gap(1)
        .data(&[
            ("B1", 2),
            ("B2", 5),
            ("B3", 7),
            ("B4", 9),
            ("B5", 12),
            ("B6", 8),
            ("B7", 5),
            ("B8", 2),
            ("B9", 7),
            ("B10", 9),
            ("B11", 12),
            ("B12", 8),
        ])
        .block(title_block("Bar Chart"));
    frame.render_widget(barchart, frame.size());
}

fn render_calendar(frame: &mut Frame) {
    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));
    let events = CalendarEventStore::default();
    let calendar = Monthly::new(
        Date::from_calendar_date(2023, time::Month::January, 1).unwrap(),
        events,
    )
    .show_month_header(Style::default())
    .default_style(default_style);
    frame.render_widget(calendar, frame.size());
}
