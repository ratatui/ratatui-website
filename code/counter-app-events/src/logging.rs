use std::{
    fmt, iter,
    sync::{Arc, RwLock},
    vec,
};

use color_eyre::Result;
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use time::{macros::format_description, OffsetDateTime};
use tracing::{
    field::{Field, Visit},
    Event, Level, Subscriber,
};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::Context, prelude::*, EnvFilter, Layer};

/// A logging layer that collects log messages
#[derive(Debug, Default)]
struct LogCollector {
    log_events: LogEvents,
}

/// A thread safe collection of log messages that can be rendered as a widget
#[derive(Debug, Default, Clone)]
pub struct LogEvents {
    logs: Arc<RwLock<Vec<LogEvent>>>,
}

/// A log message with a timestamp, target, level, message, and fields
#[derive(Debug)]
pub struct LogEvent {
    pub timestamp: OffsetDateTime,
    pub target: String,
    pub level: Level,
    pub message: String,
    pub fields: Vec<(String, String)>,
}

/// Initialize the logger with an optional log level and return the log message widget
pub fn init_logger(log_level: Option<String>) -> Result<LogEvents> {
    let log_collector = LogCollector::default();
    let logs = log_collector.log_events.clone();
    let mut env_filter = EnvFilter::from_default_env();
    // Set the log level from the command line argument
    if let Some(log_level) = log_level {
        let log_level = log_level.parse()?;
        env_filter = env_filter.add_directive(log_level);
    }
    tracing_subscriber::registry()
        .with(log_collector)
        .with(env_filter)
        .with(ErrorLayer::default()) // capture span traces for color_eyre
        .try_init()?;
    Ok(logs)
}

impl<S: Subscriber> Layer<S> for LogCollector {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let log_event = LogEvent::from(event);
        self.log_events.push(log_event);
    }
}

impl LogEvents {
    fn push(&self, log: LogEvent) {
        self.logs.write().unwrap().push(log);
    }
}

impl From<&Event<'_>> for LogEvent {
    /// Convert a tracing event into a log message
    fn from(event: &Event) -> Self {
        let metadata = event.metadata();
        let target = metadata.target();
        let level = metadata.level();
        let mut log_message = LogEvent {
            timestamp: OffsetDateTime::now_utc(),
            target: target.to_string(),
            level: *level,
            message: String::new(),
            fields: Vec::new(),
        };
        event.record(&mut log_message);
        log_message
    }
}

impl Visit for LogEvent {
    /// Extract the message and fields from the tracing event into the log message
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        } else {
            let name = field.name().to_string();
            let value = format!("{:?}", value);
            self.fields.push((name, value));
        }
    }
}

impl Widget for &LogEvents {
    /// Render the log messages as a list showing the the most recent log at the bottom
    fn render(self, area: Rect, buf: &mut Buffer) {
        let logs = self.logs.read().unwrap();
        let selected = Some(logs.len().saturating_sub(1));
        let mut state = ListState::default().with_selected(selected);
        let block = Block::default().borders(Borders::ALL).title("Logs");
        let list = List::new(logs.iter()).block(block);
        StatefulWidget::render(list, area, buf, &mut state);
    }
}

impl<'a> From<&'a LogEvent> for ListItem<'a> {
    /// Convert a log message into a list item with the timestamp, log level, target, message, and fields
    ///
    /// ```plain
    /// 02:32:25 [DEBUG] counter_app_events::events handling event
    ///   event=Redraw
    /// ````
    fn from(log: &'a LogEvent) -> Self {
        let message_line = Line::from(vec![
            timestamp_span(log.timestamp),
            log_level_span(log.level),
            format!(" {}", log.target).dim(),
            format!(" {}", log.message).white(),
        ]);
        let field_lines = log.fields.iter().map(field_line);
        let lines = iter::once(message_line).chain(field_lines).collect_vec();
        ListItem::new(lines)
    }
}

/// Create a span with the log level colored based on the log level
fn log_level_span(log_level: Level) -> Span<'static> {
    Span::styled(
        format!(" [{:5}]", log_level),
        Style::default().fg(match log_level {
            Level::ERROR => Color::Red,
            Level::WARN => Color::Yellow,
            Level::INFO => Color::Green,
            Level::DEBUG => Color::Blue,
            Level::TRACE => Color::Magenta,
        }),
    )
}

/// Create a span with the timestamp formatted as "hour:minute:second"
fn timestamp_span(timestamp: OffsetDateTime) -> Span<'static> {
    let format = format_description!("[hour]:[minute]:[second]");
    timestamp.format(&format).unwrap().dim()
}

/// Create an indented line with a field name and value
fn field_line((name, value): &(String, String)) -> Line<'_> {
    Line::styled(
        format!("    {:}={}", name, value),
        (Color::LightBlue, Modifier::ITALIC | Modifier::DIM),
    )
}
