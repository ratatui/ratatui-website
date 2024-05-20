---
title: Setup Logging with tracing
sidebar:
  order: 3
  label: Logging with Tracing
---

You'll need to install `tracing` and a few related dependencies:

```shell
cargo add tracing-error tracing
cargo add tracing-subscriber --features env-filter
cargo add directories lazy_static color-eyre # (optional)
```

You can paste the following in any module in your project.

```rust
use std::path::PathBuf;

use color_eyre::eyre::{Context, Result};
use directories::ProjectDirs;
use lazy_static::lazy_static;
use tracing::error;
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt, Layer};

lazy_static! {
  pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
  pub static ref DATA_FOLDER: Option<PathBuf> =
    std::env::var(format!("{}_DATA", PROJECT_NAME.clone())).ok().map(PathBuf::from);
  pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
  pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

fn project_directory() -> Option<ProjectDirs> {
  ProjectDirs::from("com", "kdheepak", env!("CARGO_PKG_NAME"))
}

pub fn get_data_dir() -> PathBuf {
  let directory = if let Some(s) = DATA_FOLDER.clone() {
    s
  } else if let Some(proj_dirs) = project_directory() {
    proj_dirs.data_local_dir().to_path_buf()
  } else {
    PathBuf::from(".").join(".data")
  };
  directory
}

pub fn initialize_logging() -> Result<()> {
  let directory = get_data_dir();
  std::fs::create_dir_all(directory.clone())?;
  let log_path = directory.join(LOG_FILE.clone());
  let log_file = std::fs::File::create(log_path)?;
  std::env::set_var(
    "RUST_LOG",
    std::env::var("RUST_LOG")
      .or_else(|_| std::env::var(LOG_ENV.clone()))
      .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
  );
  let file_subscriber = tracing_subscriber::fmt::layer()
    .with_file(true)
    .with_line_number(true)
    .with_writer(log_file)
    .with_target(false)
    .with_ansi(false)
    .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
  tracing_subscriber::registry().with(file_subscriber).with(ErrorLayer::default()).init();
  Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// https://github.com/tokio-rs/tracing/blob/baeba47cdaac9ed32d5ef3f6f1d7b0cc71ffdbdf/tracing-macros/src/lib.rs#L4
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


Let's say you had code like this and you want to see the value of `data`:

```rust
let table = Table::new(
    data_vec
    .iter()
    .enumerate()
    .map(|(i, data)| { // <- on this line
        let color = match i % 2 {
            0 => colors.normal_row_color,
            _ => colors.alt_row_color,
        };
        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(colors.row_fg).bg(color))
            .height(4)
    })
    .collect(),
    [
        // + 1 is for padding.
        Constraint::Length(longest_item_lens.0 + 1),
        Constraint::Min(longest_item_lens.1 + 1),
        Constraint::Min(longest_item_lens.2),
    ],
)
```

You could do

```rust
    .map(|(i, data)| { // <- to log data on this line
        tracing::info!("data = {:?}", data.clone()); // you have to add this line
        let color = match i % 2 {
            0 => colors.normal_row_color,
            _ => colors.alt_row_color,
        };
        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(colors.row_fg).bg(color))
            .height(4)
    })
```

But that requires adding code and removing code during debugging. Also, there are some instances where it 
is a lot more painful to do and you'll have to refactor your code just to print a value.

With the macro you can do this:

```rust
    .map(|(i, trace_dbg!(data))| { // <- on this line
        let color = match i % 2 {
            0 => colors.normal_row_color,
            _ => colors.alt_row_color,
        };
        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(colors.row_fg).bg(color))
            .height(4)
    })
```

i.e. `trace_dbg!(data)` is like `dbg!(data)` but uses logging with `tracing` instead of `stdout`.

Make sure you call `initialize_logging()?` in your `main()` function.

The log level is decided by the `${YOUR_CRATE_NAME}_LOGLEVEL` environment variable (default =
`log::LevelFilter::Info`).

Additionally, the location of the log files would be decided by your environment variables. See
[the section on XDG directories](../config-directories/) for more information.

:::tip

Check out [`tui-logger`](https://github.com/gin66/tui-logger) for setting up a tui logger widget
with tracing.

:::

![Top half is a terminal with the TUI showing a Vertical split with tui-logger widget. Bottom half is a terminal showing the output of running `tail -f` on the log file.](https://user-images.githubusercontent.com/1813121/254093932-46d8c6fd-c572-4675-bcaf-45a36eed51ff.png)
