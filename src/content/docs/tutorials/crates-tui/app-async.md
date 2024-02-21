---
title: App Async
---

We are finally ready to incorporate the helper module into the `App` struct.

Define the the following fields in the `App` struct:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app}}
```

We already saw that we needed a `Arc<Mutex<Vec<crates_io_api::Crate>>>` for getting results. Let's
use [`tui-input`] for handling the search prompt and a `Option<Position>` to handle displaying the
cursor in the prompt.

[`tui-input`]: https://github.com/sayanarijit/tui-input

Let's also add a `TableState` for allowing scrolling in the results.

For the application, we want to be able to:

**In prompt mode**:

1. Type any character into the search prompt
2. Hit Enter to submit a search query
3. Hit Esc to return focus to the results view

**In results mode**:

1. Use arrow keys to scroll
2. Use `/` to enter search mode
3. Use Esc to quit the application

Expand the `handle_events` to the match on mode and change the app state accordingly:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app_handle_event}}
}
```

`tui-input` handles events for moving the cursor in the prompt.

## Submit search query

`tui-input` has a [`Input::value`] method that you can use to get a reference to the current search
query that the user has typed in, i.e. `self.prompt.value() -> &str`.

[`Input::value`]: https://docs.rs/tui-input/latest/tui_input/struct.Input.html#method.value

Implement the following method:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app_submit_search_query}}
}
```

:::tip

You can call `tokio::spawn` in a normal function and it will spin up a task in the background for
execution.

:::

## Scroll up and Scroll down

When the `scroll_up` or `scroll_down` methods are called, you have to update the `TableState` of the
results to select the new index.

Implement the following for wrapped scrolling:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app_scroll}}
}
```

## Cursor state

Ratatui hides the cursor by default every frame. To show it, we have to call `set_cursor`
explicitly. We only want to show the cursor when the prompt is in focus.

Implement the following to show the cursor conditionally:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app_draw}}

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app_update_cursor}}

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:update_prompt_cursor_state}}
}
```

## Draw

Finally, you can update the render using the new information to replace placeholder data with the
data from the results or the prompt value.

### Results

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app_results_table_widget}}
}
```

Note the use `highlight_symbol` here to show the cursor when scrolling.

### Prompt

Update the prompt widget to show the text from `tui-input::Input` in a `Paragraph` widget:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app_prompt_widget}}
}
```

### Render

And in the render function for the `StatefulWidget`, make sure you create a stateful widget for the
table results instead. You have to also call the function that updates the cursor position based on
the prompt `Rect`, which is only known during render.

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:app_statefulwidget}}
```

## Conclusion

Here's the full app for your reference:

```rust collapsed title="src/app.rs (click to expand)"
use color_eyre::Result;
use itertools::Itertools;
use ratatui::layout::Position;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::{
    events::{Event, Events},
    tui::Tui
};

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-async.rs:full_app}}
```
