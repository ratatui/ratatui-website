# Summary

[Introduction to Ratatui](./README.md)

- [Installation](./installation.md)

- [Tutorials](./tutorial/README.md)

  - [Hello World](./tutorial/hello-world/README.md)
  - [Counter App](./tutorial/counter-app/README.md)
    - [Single Function](./tutorial/counter-app/single-function.md)
    - [Multiple Functions](./tutorial/counter-app/multiple-functions.md)
    - [Multiple Files](./tutorial/counter-app/multiple-files.md)
      - [app.rs](./tutorial/counter-app/app.md)
      - [ui.rs](./tutorial/counter-app/ui.md)
      - [event.rs](./tutorial/counter-app/event.md)
      - [tui.rs](./tutorial/counter-app/tui.md)
      - [update.rs](./tutorial/counter-app/update.md)
      - [main.rs](./tutorial/counter-app/main.md)
  - [JSON Editor](./tutorial/json-editor/README.md)
    - [App.rs - Holding application state](./tutorial/json-editor/app.md)
    - [Main.rs - UI loop and event handling](./tutorial/json-editor/main.md)
    - [Ui.rs - Layouts, widgets, frames, oh my!](./tutorial/json-editor/ui.md)
      - [Ui.rs - Main](./tutorial/json-editor/ui-main.md)
      - [Ui.rs - Editing](./tutorial/json-editor/ui-editing.md)
      - [Ui.rs - Exit](./tutorial/json-editor/ui-exit.md)
    - [Conclusion](./tutorial/json-editor/closing_thoughts.md)
  - [Async Counter App](./tutorial/counter-async-app/README.md)
    - [Async KeyEvents](./tutorial/counter-async-app/async-event-stream.md)
    - [Async Render](./tutorial/counter-async-app/full-async-events.md)
    - [Introducing Actions](./tutorial/counter-async-app/actions.md)
    - [Async Actions](./tutorial/counter-async-app/full-async-actions.md)
    - [Conclusion](./tutorial/counter-async-app/conclusion.md)
  - [Stopwatch App](./tutorial/stopwatch-app/README.md)

- [Concepts](./concepts/README.md)

  - [Rendering](./concepts/rendering.md)
    - [Under the hood](./concepts/rendering-under-the-hood.md)
  - [Layout](./concepts/layout/README.md)
  - [Application Patterns](./concepts/application-patterns/README.md)
    - [The Elm Architecture](./concepts/application-patterns/the-elm-architecture.md)
    - [Component Architecture](./concepts/application-patterns/component-architecture.md)
    - [Flux Architecture](./concepts/application-patterns/flux-architecture.md)
  - [Backends](./concepts/backends/README.md)
    - [Comparison](./concepts/backends/comparison.md)
    - [Raw Mode](./concepts/backends/raw-mode.md)
    - [Alternate Screen](./concepts/backends/alternate-screen.md)
    - [Mouse Capture](./concepts/backends/mouse-capture.md)
  - [Event Handling](./concepts/event_handling.md)

- [How To](./how-to/README.md)

  - [Layout UIs](./how-to/layout/README.md)
    - [Dynamic Layouts](./how-to/layout/dynamic.md)
    - [Center a Rect](./how-to/layout/center-a-rect.md)
    - [Collapse Borders](./how-to/layout/collapse-borders.md)
  - [Render UIs](./how-to/render/README.md)
    - [Display Text](./how-to/render/display-text.md)
    - [Style Text](./how-to/render/style-text.md)
    - [Overwrite Regions](./how-to/render/overwrite-regions.md)
  - [Use Widgets](./how-to/widgets/README.md)
    - [Paragraph](./how-to/widgets/paragraph.md)
    - [Block](./how-to/widgets/block.md)
    - [Custom](./how-to/widgets/custom.md)
  - [Develop Applications](./how-to/develop-apps/README.md)
    - [CLI arguments](./how-to/develop-apps/cli-arguments.md)
    - [Configuration Directories](./how-to/develop-apps/config-directories.md)
    - [Logging with Tracing](./how-to/develop-apps/tracing.md)
    - [Async Terminal and Event handler](./how-to/develop-apps/abstract-terminal-and-event-handler.md)
    - [Setup Panic Hooks](./how-to/develop-apps/setup-panic-hooks.md)
    - [Better Panic Hooks](./how-to/develop-apps/better-panic-hooks.md)
    - [Migrate from tui-rs](./how-to/develop-apps/migrate-from-tui-rs.md)

- [FAQ](./faq/README.md)

  - [Duplicate key events](./faq/duplicate-key-events-windows.md)
  - [`tokio` / `async`](./faq/tokio-async.md)
  - [`tui.rs` history](./faq/tui-rs-history.md)

- [Highlights]()

  - [v0.24](./highlights/v0.24.md)
  - [v0.23](./highlights/v0.23.md)
  - [v0.22](./highlights/v0.22.md)
  - [v0.21](./highlights/v0.21.md)

- [References](./references/README.md)

  - [Showcase](./showcase/README.md)
  - [Features](./references/features.md)

- [Developer Guide]()

  - [Ratatui](./developer-guide/ratatui.md)
  - [Ratatui Book](./developer-guide/book.md)
  - [License](./LICENSE.md)

---

[Contributors](contributors.md)
