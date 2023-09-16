# Use color-eyre and human-panic

### color-eyre panic hook

One way to manage printing of stack-traces is by using
[`color-eyre`](https://github.com/eyre-rs/color-eyre):

```console
cargo add color-eyre
```

You will also want to add a `repository` key to your `Cargo.toml` file:

```toml
repository = "https://github.com/ratatui-org/ratatui-async-template" # used by env!("CARGO_PKG_REPOSITORY")
```

Now, let's say I added a `panic!` to my application as an example:

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

Then when this function is called, I can have the application cleanly restore the terminal and print
out a nice error message like so:

```
The application panicked (crashed).
Message:  At the disco
Location: src/components/app.rs:80

This is a bug. Consider reporting it at https://github.com/ratatui-org/ratatui-async-template

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```

Users can opt to give you a more detailed stacktrace if they can reproduce the error with
`export RUST_BACKTRACE=1`:

```
The application panicked (crashed).
Message:  At the disco
Location: src/components/app.rs:80

This is a bug. Consider reporting it at https://github.com/ratatui-org/ratatui-async-template

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                                ⋮ 13 frames hidden ⋮
  14: ratatui_async_template::components::app::App::increment::h4e8b6e0d83d3d575
      at /Users/kd/gitrepos/ratatui-async-template/src/components/app.rs:80
  15: <ratatui_async_template::components::app::App as ratatui_async_template::components::Component>::update::hc78145b4a91e06b6
      at /Users/kd/gitrepos/ratatui-async-template/src/components/app.rs:132
  16: ratatui_async_template::runner::Runner::run::{{closure}}::h802b0d3c3413762b
      at /Users/kd/gitrepos/ratatui-async-template/src/runner.rs:80
  17: ratatui_async_template::main::{{closure}}::hd78d335f19634c3f
      at /Users/kd/gitrepos/ratatui-async-template/src/main.rs:44
  18: tokio::runtime::park::CachedParkThread::block_on::{{closure}}::hd7949515524de9f8
      at /Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/park.rs:283
  19: tokio::runtime::coop::with_budget::h39648e20808374d3
      at /Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/coop.rs:107
  20: tokio::runtime::coop::budget::h653c1593abdd982d
      at /Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/coop.rs:73
  21: tokio::runtime::park::CachedParkThread::block_on::hb0a0dd4a7c3cf33b
      at /Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/park.rs:283
  22: tokio::runtime::context::BlockingRegionGuard::block_on::h4d02ab23bd93d0fd
      at /Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/context.rs:315
  23: tokio::runtime::scheduler::multi_thread::MultiThread::block_on::h8aaba9030519c80d
      at /Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/scheduler/multi_thread/mod.rs:66
  24: tokio::runtime::runtime::Runtime::block_on::h73a6fbfba201fac9
      at /Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/runtime.rs:304
  25: ratatui_async_template::main::h6da543b193746523
      at /Users/kd/gitrepos/ratatui-async-template/src/main.rs:46
  26: core::ops::function::FnOnce::call_once::h6cac3edc975fcef2
      at /rustc/eb26296b556cef10fb713a38f3d16b9886080f26/library/core/src/ops/function.rs:250
                                ⋮ 13 frames hidden ⋮
```

## human-panic

Personally, I like to use `human-panic` to print out a user friendly message like so:

```
The application panicked (crashed).
Message:  At the disco
Location: src/components/app.rs:80

This is a bug. Consider reporting it at https://github.com/ratatui-org/ratatui-async-template

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
Well, this is embarrassing.

ratatui-async-template had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "/var/folders/l4/bnjjc6p15zd3jnty8c_qkrtr0000gn/T/report-ce1e29cb-c17c-4684-b9d4-92d9678242b7.toml". Submit an issue or email with the subject of "ratatui-async-template Crash Report" and include the report as an attachment.

- Authors: Dheepak Krishnamurthy

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!
```

Here's the content of the temporary report file that `human-panic` creates:

```
name = "ratatui-async-template"
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

You'll need [human-panic](https://github.com/rust-cli/human-panic) installed as a dependency for
this:

```console
cargo add human-panic
```

## Configuration

You can mix and match different panic handlers, using `better-panic` for debug builds and
`color-eyre` and `human-panic` for release builds. The code below also prints the `color-eyre`
stacktrace to `log::error!` for good measure (after striping ansi escape sequences).

```console
cargo add color-eyre human-panic libc better-panic strip-ansi-escapes
```

Here's code you can copy paste into your project (if you use the
[`Tui`](./abstract-terminal-and-event-handler.md) struct to handle terminal exits):

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
      eprintln!("{}", msg); // prints color-eyre stack trace to stderr
      use human_panic::{handle_dump, print_msg, Metadata};
      let meta = Metadata {
        version: env!("CARGO_PKG_VERSION").into(),
        name: env!("CARGO_PKG_NAME").into(),
        authors: env!("CARGO_PKG_AUTHORS").replace(':', ", ").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
      };

      let file_path = handle_dump(&meta, panic_info);
      // prints human-panic message
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
