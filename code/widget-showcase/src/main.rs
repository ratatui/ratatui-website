use std::{panic, time::Duration};

use clap::{Parser, ValueEnum};
use color_eyre::{
    config::HookBuilder,
    eyre::{self, WrapErr},
};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    Frame, Terminal,
};

mod examples;
mod tui;

use examples::{
    bar_chart, block, calendar, canvas, chart, gauge, line_gauge, list, paragraph, sparkline,
    table, tabs,
};

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
    Canvas,
    Chart,
    Gauge,
    LineGauge,
    List,
    Paragraph,
    Sparkline,
    Table,
    Tabs,
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
    fn render_frame(&self, frame: &mut Frame, widget: Widget) -> color_eyre::Result<()> {
        match widget {
            Widget::Block => block::render(frame),
            Widget::BarChart => bar_chart::render(frame),
            Widget::Calendar => crate::calendar::render(frame)?,
            Widget::Canvas => canvas::render(frame),
            Widget::Chart => chart::render(frame),
            Widget::Gauge => gauge::render(frame),
            Widget::LineGauge => line_gauge::render(frame),
            Widget::List => list::render(frame),
            Widget::Paragraph => paragraph::render(frame),
            Widget::Sparkline => sparkline::render(frame),
            Widget::Table => table::render(frame),
            Widget::Tabs => tabs::render(frame),
        }
        Ok(())
    }

    fn update(&mut self) -> color_eyre::Result<()> {
        // quit if a timeout occurs
        if !event::poll(Duration::from_secs(3))? {
            self.running_state = RunningState::Finished;
        }
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event::read()?
        {
            match code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.running_state = RunningState::Finished;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
