---
title: Widgets
---

In this section we will discuss implementing widgets.

Create a new file `./src/widgets.rs` with the following content:

```rust title="src/widgets.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets.rs}}
```

We will be making a `SearchPage` widget that composes a `SearchResults` widget and a `SearchPrompt`
widget.

![](./crates-tui-demo-1.png)

For the `SearchResults`, we will use a `Table` like before, and additionally a `Scrollbar` widget.
For the `SearchPrompt`, we will use a `Block` with borders and `Paragraph`s for the text like
before.

We will be using the `StatefulWidget` pattern. `StatefulWidget` is a trait in Ratatui that is
defined like so:

```rust
pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
```

For this `StatefulWidget` pattern, you will always have at a minimum two `struct`s for every widget:

1. the state
2. the widget

You used this pattern already in the `app` module with the `App` struct as the state and the
`AppWidget` struct as the widget that is rendered. Now you are going to apply it to refactor the
`App` into children.
