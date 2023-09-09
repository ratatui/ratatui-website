# Backends

Ratatui interfaces with the terminal emulator through a backend. These libraries enable Ratatui via
the [`Terminal`] type to draw styled text to the screen, manipulate the cursor, and interrogate
properties of the terminal such as the console or window size. You application will generally also
use the backend directly to capture keyboard, mouse and window events, and enable raw mode and the
alternate screen.

Ratatui supports the following backends:

- [Crossterm] via [`CrosstermBackend`] and the `crossterm` (enabled by default).
- [Termion] via [`TermionBackend`] and the `termion` feature.
- [Termwiz] via [`TermwizBackend`] and the `termion` feature.
- A [`TestBackend`] which can be useful to unit test your application's UI

For information on how to choose a backend see: [Comparison](./comparison.md)

Each backend supports [Raw Mode](./raw-mode.md) (which changes how the terminal handles input and
output processing), an [Alternate Screen](./alternate-screen.md) which allows it to render to a
separate buffer than your shell commands use, and [Mouse Capture](./mouse-capture.md), which allows
your application to capture mouse events.

[Crossterm]: https://crates.io/crate/crossterm
[Termion]: https://crates.io/crate/termion
[Termwiz]: https://crates.io/crate/termwiz
[`Terminal`]: https://docs.rs/ratatui/latest/ratatui/terminal/struct.Terminal.html
[`CrosstermBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html
[`TermionBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TermionBackend.html
[`TermwizBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TermwizBackend.html
[`TestBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html
