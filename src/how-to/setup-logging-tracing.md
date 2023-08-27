# Setup Logging with tracing

You can paste the following in any module in your project. Call `initialize_logging()?` in your
`main()` function.

```rust
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use tracing::error;
use tracing_subscriber::{
  self, filter::EnvFilter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};

pub fn initialize_logging() -> Result<()> {
  let directory = PathBuf::from("./log/");
  std::fs::create_dir_all(directory.clone()).context(format!("{directory:?} could not be created"))?;
  let log_path = directory.join("ratatui-app.log");
  let log_file = std::fs::File::create(log_path)?;
  let file_subscriber = tracing_subscriber::fmt::layer()
    .with_file(true)
    .with_line_number(true)
    .with_writer(log_file)
    .with_target(false)
    .with_ansi(false)
    .with_filter(EnvFilter::from_default_env());
  tracing_subscriber::registry().with(file_subscriber).with(tui_logger::tracing_subscriber_layer()).init();
  let default_level = std::env::var("RUST_LOG").map_or(log::LevelFilter::Info, |val| {
    match val.to_lowercase().as_str() {
      "off" => log::LevelFilter::Off,
      "error" => log::LevelFilter::Error,
      "warn" => log::LevelFilter::Warn,
      "info" => log::LevelFilter::Info,
      "debug" => log::LevelFilter::Debug,
      "trace" => log::LevelFilter::Trace,
      _ => log::LevelFilter::Info,
    }
  });
  tui_logger::set_default_level(default_level);
  Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr, level: $level:expr, $ex:expr) => {{
        match $ex {
            value => {
                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                value
            }
        }
    }};
    (level: $level:expr, $ex:expr) => {
        trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
        trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}

```

The log level is decided by the `RUST_LOG` environment variable (default =
`log::LevelFilter::Info`).

Ideally, the location of the log files are decided by your environment variables. See
[the section on XDG directories](./handle-xdg-directories.md) for how to handle that.

In addition to add a log file to the `data` folder, `initialize_logging()` also sets up `tui-logger`
with `tracing`, so that you can add a `tui-logger` widget to show the logs to your users on a key
press.

![Top half is a terminal with the TUI showing a Vertical split with tui-logger widget. Bottom half is a terminal showing the output of running `tail -f` on the log file.](https://user-images.githubusercontent.com/1813121/254093932-46d8c6fd-c572-4675-bcaf-45a36eed51ff.png)
