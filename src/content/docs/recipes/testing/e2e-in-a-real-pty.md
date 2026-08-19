---
title: End-to-end testing in a real PTY
sidebar:
  order: 3
---

[Snapshot tests with `TestBackend`](/recipes/testing/snapshots/) are great for widgets: they render
in-process and never touch a terminal. What they can't cover is the rest of your app — the event
loop, key handling, terminal setup and teardown — because none of that runs. A PTY test harness
such as [termlens] fills that gap: it spawns your **compiled binary** in a real pseudo-terminal,
feeds its output through a VT emulator, and lets tests assert or snapshot on the rendered screen,
CI included (no TTY needed — the harness creates its own).

The two approaches are complementary: keep `TestBackend` snapshots for widget-level units, and add
a handful of PTY tests for whole-app flows.

### 1. Add dependencies

```shell
cargo add termlens --dev
cargo add insta --dev
```

### 2. An app to test

Any binary works unmodified. Here's the counter app in the shape of the
[basic app tutorial](/tutorials/counter-app/basic-app/): `j` increments, `k` decrements, `q` quits.

```rust
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::Paragraph;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut counter = 0u32;
    let result = loop {
        let draw = terminal.draw(|frame| {
            frame.render_widget(Paragraph::new(format!("Counter: {counter}")), frame.area());
        });
        if let Err(err) = draw {
            break Err(err);
        }
        match event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('j') => counter += 1,
                KeyCode::Char('k') => counter = counter.saturating_sub(1),
                KeyCode::Char('q') => break Ok(()),
                _ => {}
            },
            Ok(_) => {}
            Err(err) => break Err(err),
        }
    };
    ratatui::restore();
    result
}
```

### 3. The test

`env!("CARGO_BIN_EXE_<name>")` points at your compiled binary, so the test drives exactly what
your users run:

```rust
use std::time::Duration;
use termlens::{Key, Terminal};

#[test]
fn counts_up_and_quits() -> termlens::Result<()> {
    let mut t = Terminal::builder()
        .size(80, 24)
        .env_clear() // hermetic: the host environment can't change a snapshot
        .timeout(Duration::from_secs(5))
        .spawn(env!("CARGO_BIN_EXE_counter-app"))?;

    t.wait_until(|screen| screen.contains("Counter: 0"))?;

    t.send(Key::Char('j'));
    t.send(Key::Char('j'));
    t.wait_until(|screen| screen.contains("Counter: 2"))?;
    insta::assert_snapshot!(t.screen());

    t.send(Key::Char('q'));
    assert!(t.wait_exit()?.success());
    Ok(())
}
```

The stored snapshot is the rendered grid, so a review shows what a user would see:

```text
size: 80x24  cursor: hidden
Counter: 2
```

Every `wait_*` call runs under a deadline, and a timeout error embeds the full screen dump — a
failing CI log shows what the app was actually displaying, not just `assertion failed`.

### 4. Waiting deterministically

PTY output is asynchronous, so never assert immediately after sending a key — wait for its visible
effect first (`wait_until` above). Before snapshotting a whole screen, wait on the **last** thing
the app draws in that frame, so the snapshot can't catch a partially processed update. For "the
app settled" situations there is also `wait_idle`, a quiet-period heuristic; prefer waiting on
content when you can.

### When to use which

| | `TestBackend` snapshots | PTY end-to-end |
|---|---|---|
| Renders | one widget, in-process | your real binary in a real terminal |
| Covers | layout and styling logic | event loop, key handling, setup/teardown, exit codes |
| Speed | fastest | fast (one process spawn per test, ~ms) |
| Best for | many fine-grained unit tests | a few whole-app flow tests |

[termlens]: https://github.com/vyncint/termlens
