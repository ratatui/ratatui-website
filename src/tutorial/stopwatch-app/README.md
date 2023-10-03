# Stopwatch App

Here's the dependencies:

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
