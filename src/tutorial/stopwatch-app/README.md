# Stopwatch App

In this section, we are going to combine what we learnt in the previous tutorials and build a
stopwatch application. We are also going to take advantage of a widget from an external dependency.

Here's the dependencies you'll need in your `Cargo.toml`:

```toml
{{#include ./ratatui-stopwatch-app/Cargo.toml}}
```

Here's a gif of what it will look like if you run this:

![Stopwatch](https://vhs.charm.sh/vhs-3dTTtrLkyU54hNah22PAR9.gif)

This application uses an external dependency called
[`tui-big-text`](https://github.com/joshka/tui-big-text).

This application also combines the `AppState` (or Mode) pattern from
[the JSON Editor](../json-editor/README.md) with the `Message` (or `Command` or `Action`) pattern
from [the Async Counter App](../counter-async-app/README.md). This `Message` pattern is common in
[The Elm Architecture pattern](../../concepts/application-patterns/the-elm-architecture.md).

This application uses a `Tui` struct that
[combines the `Terminal` and `Event Handler`](../../how-to/develop-apps/abstract-terminal-and-event-handler.md).

The full code is available on
[GitHub](https://github.com/ratatui-org/ratatui-book/tree/main/src/tutorial/stopwatch-app/ratatui-stopwatch-app).

Here's the relevant application part of the code:

```rust
{{#include ./ratatui-stopwatch-app/src/main.rs:app}}
```

It is worth thinking about what it takes to build your own custom widget by looking at
[the source for the `BigText` widget](https://github.com/joshka/tui-big-text/blob/7f9e84968a9ba4db824a7ece7d186e58bb35999d/src/lib.rs#L83-L104):

```rust
#[derive(Debug, Builder, Clone, PartialEq, Eq, Hash)]
pub struct BigText<'a> {
    #[builder(setter(into))]
    lines: Vec<Line<'a>>,

    #[builder(default)]
    style: Style,
}

impl Widget for BigText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = layout(area);
        for (line, line_layout) in self.lines.iter().zip(layout) {
            for (g, cell) in line.styled_graphemes(self.style).zip(line_layout) {
                render_symbol(g, cell, buf);
            }
        }
    }
}
```

To build a custom widget, you have ti implement the `Widget` trait. We cover how to implement the
`Widget` trait for your own structs in [a separate section](../../how-to/widgets/custom.md).
