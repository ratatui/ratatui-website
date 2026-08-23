---
title: Backends
sidebar:
  order: 0
---

Ratatui interfaces with the terminal emulator through a backend. These libraries enable Ratatui via
the [`Terminal`] type to draw styled text to the screen, manipulate the cursor, and interrogate
properties of the terminal such as the console or window size. Your application will generally also
use the backend directly to capture keyboard, mouse and window events, and enable raw mode and the
alternate screen.

Ratatui supports the following backends:

- [Crossterm] via [`CrosstermBackend`] and the `crossterm` feature (enabled by default). Also see
  [Crossterm version compatibility](#crossterm-version-compatibility) below for details on selecting
  specific versions.
- [Termion] via [`TermionBackend`] and the `termion` feature.
- [Termwiz] via [`TermwizBackend`] and the `termwiz` feature.
- [Termina] via [`TerminaBackend`] and the `termina` feature.
- A [`TestBackend`] which can be useful to unit test your application's UI

For information on how to choose a backend see: [Comparison](./comparison/)

Each backend supports [Raw Mode](./raw-mode/) (which changes how the terminal handles input and
output processing), an [Alternate Screen](./alternate-screen/) which allows it to render to a
separate buffer than your shell commands use, and [Mouse Capture](./mouse-capture/), which allows
your application to capture mouse events.

## Crossterm Version Compatibility

Avoid pulling in multiple semver-incompatible [Crossterm] versions. Different major versions:

- keep separate event queues (which can lead to race conditions and lost events),
- track raw mode separately (so raw mode may not be restored correctly on exit),
- cannot exchange types even when names match (leading to compilation errors).

Also, specific versions may make it difficult to upgrade Ratatui/widgets unless everything is up to
date.

As a mitigation, the split `ratatui-crossterm` package supports multiple [Crossterm] major versions
through `crossterm_{version}` feature flags. The main `ratatui` package uses Crossterm 0.29 by
default.

Ratatui 0.30.2 uses Crossterm 0.29 by default:

```toml
ratatui = "0.30"
crossterm = "0.29"
```

To select Crossterm 0.28, use the split backend package directly. Disable its defaults so Crossterm
0.29 is not also enabled:

```toml
crossterm = "0.28"
ratatui = { version = "0.30", default-features = false }
ratatui-crossterm = { version = "0.1", default-features = false, features = ["crossterm_0_28"] }
```

:::note

- If multiple flags are enabled, Ratatui selects the latest.
- This uses `ratatui_crossterm::CrosstermBackend`; the main `ratatui` package's Crossterm setup
  helpers are not enabled.
- Disabling Ratatui's default features also disables its other default features. Re-enable the ones
  the application needs.
- The main `ratatui` package forwards the version flags, but activating its backend also activates
  `ratatui-crossterm`'s default version.
- Use `cargo tree -p crossterm` to check your graph and disable default features on dependencies
  that drag in another Crossterm major.
- Ratatui 0.30+ introduces `ratatui-core`, moving backends into separate crates so backend changes
  can evolve independently of the main library. This also helps avoid version conflicts in
  applications that only need one backend.

:::

[Crossterm]: https://crates.io/crates/crossterm
[Termion]: https://crates.io/crates/termion
[Termwiz]: https://crates.io/crates/termwiz
[Termina]: https://crates.io/crates/termina
[`Terminal`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html
[`CrosstermBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html
[`TermionBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TermionBackend.html
[`TermwizBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TermwizBackend.html
[`TerminaBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TerminaBackend.html
[`TestBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html
