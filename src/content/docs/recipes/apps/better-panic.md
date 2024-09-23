---
title: Better Panic Hooks
sidebar:
  order: 7
---

Your application may panic for a number of reasons (e.g. when you call `.unwrap()` on a `None`). And
when this happens, you want to be a good citizen and:

1. provide a useful stacktrace so that they can report errors back to you.
2. not leave the users terminal state in a botched condition, resetting it back to the way it was.

## `better-panic`

[`better-panic`](https://github.com/mitsuhiko/better-panic) gives you pretty backtraces for panics.

```shell
cargo add better-panic
```

Here's an example of `initialize_panic_handler()` using `better-panic` to provide a prettier
backtrace by default.

```rust
use better_panic::Settings;

pub fn initialize_panic_handler() {
  std::panic::set_hook(Box::new(|panic_info| {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
    Settings::auto().most_recent_first(false).lineno_suffix(true).create_panic_handler()(panic_info);
  }));
}
```

I personally like to reuse the [`Tui`](../terminal-and-event-handler/) struct in the panic handler.
That way, if I ever decide to move from `crossterm` to `termion` in the future, there's one less
place in the project that I have to worry about refactoring.

Here's an example of `initialize_panic_handler()` using
[`better_panic`](https://docs.rs/better-panic/latest/better_panic/) and
[`libc`](https://docs.rs/libc/latest/libc/) to provide a prettier backtrace by default.

```rust
use better_panic::Settings;

pub fn initialize_panic_handler() {
  std::panic::set_hook(Box::new(|panic_info| {
    match crate::tui::Tui::new() {
      Ok(t) => {
        if let Err(r) = t.exit() {
          error!("Unable to exit Terminal: {r:?}");
        }
      },
      Err(r) => error!("Unable to exit Terminal: {r:?}"),
    }
    better_panic::Settings::auto()
      .most_recent_first(false)
      .lineno_suffix(true)
      .verbosity(better_panic::Verbosity::Full)
      .create_panic_handler()(panic_info);
    std::process::exit(libc::EXIT_FAILURE);
  }));
}
```

Now, let's say I added a `panic!` to an application as an example:

```diff
diff --git a/src/components/app.rs b/src/components/app.rs
index 289e40b..de48392 100644
--- a/src/components/app.rs
+++ b/src/components/app.rs
@@ -77,6 +77,7 @@ impl App {
   }

   pub fn increment(&mut self, i: usize) {
+    panic!("At the disco");
     self.counter = self.counter.saturating_add(i);
   }
```

This is what a prettier stacktrace would look like with `better-panic`:

```
Backtrace (most recent call last):
  File "/Users/kd/gitrepos/myapp/src/main.rs:46", in ratatui_async_template::main
    Ok(())
  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/runtime.rs:304", in tokio::runtime::runtime::Runtime::block_on
    Scheduler::MultiThread(exec) => exec.block_on(&self.handle.inner, future),
  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/scheduler/multi_thread/mod.rs:66", in tokio::runtime::scheduler::multi_thread::MultiThread::block_on
    enter
  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/context.rs:315", in tokio::runtime::context::BlockingRegionGuard::block_on
    park.block_on(f)
  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/park.rs:283", in tokio::runtime::park::CachedParkThread::block_on
    if let Ready(v) = crate::runtime::coop::budget(|| f.as_mut().poll(&mut cx)) {
  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/coop.rs:73", in tokio::runtime::coop::budget
    with_budget(Budget::initial(), f)
  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/coop.rs:107", in tokio::runtime::coop::with_budget
    f()
  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/park.rs:283", in tokio::runtime::park::CachedParkThread::block_on::{{closure}}
    if let Ready(v) = crate::runtime::coop::budget(|| f.as_mut().poll(&mut cx)) {
  File "/Users/kd/gitrepos/myapp/src/main.rs:44", in ratatui_async_template::main::{{closure}}
    runner.run().await?;
  File "/Users/kd/gitrepos/myapp/src/runner.rs:80", in ratatui_async_template::runner::Runner::run::{{closure}}
    if let Some(action) = component.update(action.clone())? {
  File "/Users/kd/gitrepos/myapp/src/components/app.rs:132", in <ratatui_async_template::components::app::App as ratatui_async_template::components::Component>::update
    Action::Increment(i) => self.increment(i),
  File "/Users/kd/gitrepos/myapp/src/components/app.rs:80", in ratatui_async_template::components::app::App::increment
    panic!("At the disco");

The application panicked (crashed).
  At the disco
in src/components/app.rs:80
thread: main
```

With `.most_recent_first(false)` the last line of the stacktrace is typically where the error has
occurred. This makes it fast and easy to find the error without having to scroll up the terminal
history, and iterate on your application rapidly during development.

This kind of detailed stacktrace is only available in debug builds. For release builds, you may get
inlined or truncated stacktraces.

For example, here's what I get when I compile with all optimizations on:

```
Backtrace (most recent call last):
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header
  File "<unknown>:0", in __mh_execute_header

The application panicked (crashed).
  At the disco
in src/components/app.rs:80
thread: main
```

This is not particularly useful to show to the average user. We'll discuss better solutions for what
to show the users of your application in the following subsections.

## human-panic

To use [human-panic](https://github.com/rust-cli/human-panic), you'll have to install it as a
dependency:

```shell
cargo add human-panic
```

Personally, I think `human-panic` provides the most user friendly panic handling functionality out
of the box when users experience an unexpected panic:

```
Well, this is embarrassing.

myapp had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "/var/folders/l4/bnjjc6p15zd3jnty8c_qkrtr0000gn/T/report-ce1e29cb-c17c-4684-b9d4-92d9678242b7.toml". Submit an issue or email with the subject of "myapp Crash Report" and include the report as an attachment.

- Authors: Dheepak Krishnamurthy

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!
```

It generates a report where information relevant to the crash is logged. Here's the content of the
temporary report file that `human-panic` creates (with optimizations turned on):

```
name = "myapp"
operating_system = "Mac OS 13.5.2 [64-bit]"
crate_version = "0.1.0"
explanation = """
Panic occurred in file 'src/components/app.rs' at line 80
"""
cause = "At the disco"
method = "Panic"
backtrace = """

   0: 0x10448f5f8 - __mh_execute_header
   1: 0x1044a43c8 - __mh_execute_header
   2: 0x1044a01ac - __mh_execute_header
   3: 0x10446f8c0 - __mh_execute_header
   4: 0x1044ac850 - __mh_execute_header"""
```

In debug mode, the stacktrace is as descriptive as earlier.

## Configuration

You can mix and match these different panic handlers, using `better-panic` for debug builds and
`color-eyre` and `human-panic` for release builds. The code below also prints the `color-eyre`
stacktrace to `log::error!` for good measure (after striping ansi escape sequences).

```shell
cargo add color-eyre human-panic libc better-panic strip-ansi-escapes
```

Here's code you can copy paste into your project (if you use the
[`Tui`](./terminal-and-event-handler/) struct to handle terminal exits):

```rust
pub fn initialize_panic_handler() -> Result<()> {
  let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
    .panic_section(format!("This is a bug. Consider reporting it at {}", env!("CARGO_PKG_REPOSITORY")))
    .display_location_section(true)
    .display_env_section(true)
    .into_hooks();
  eyre_hook.install()?;
  std::panic::set_hook(Box::new(move |panic_info| {
    if let Ok(t) = crate::tui::Tui::new() {
      if let Err(r) = t.exit() {
        error!("Unable to exit Terminal: {:?}", r);
      }
    }

    let msg = format!("{}", panic_hook.panic_report(panic_info));
    #[cfg(not(debug_assertions))]
    {
        eprintln!("{msg}");
        use human_panic::{handle_dump, print_msg, Metadata};
        let author = format!("authored by {}", env!("CARGO_PKG_AUTHORS"));
        let support = format!(
            "You can open a support request at {}",
            env!("CARGO_PKG_REPOSITORY")
        );
        let meta = Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors(author)
            .support(support);

        let file_path = handle_dump(&meta, panic_info);
        print_msg(file_path, &meta).expect("human-panic: printing error message to console failed");
    }
    log::error!("Error: {}", strip_ansi_escapes::strip_str(msg));

    #[cfg(debug_assertions)]
    {
      // Better Panic stacktrace that is only enabled when debugging.
      better_panic::Settings::auto()
        .most_recent_first(false)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .create_panic_handler()(panic_info);
    }

    std::process::exit(libc::EXIT_FAILURE);
  }));
  Ok(())
}
```
