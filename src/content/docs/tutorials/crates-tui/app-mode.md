---
title: App Mode
---

In this section, you are going to expand on the `App` struct to add a `Mode`.

## App

Define the following fields in the `App` struct:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-mode.rs:app}}
```

## Mode

Our app is going to have two focus modes:

1. when the `Prompt` is in focus,

   ![](./crates-tui-demo-1.png)

2. when the `Results` are in focus.

   ![](./crates-tui-demo-2.png)

You can represent the state of the "focus" using an enum called `Mode`:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-mode.rs:mode}}
```

The reason you want to do this is because you may want to do different things when receiving the
same event in different modes. For example, `ESC` when the prompt is in focus should switch the mode
to results, but `ESC` when the results are in focus should exit the app.

## App::handle_event

Change the `handle_event` function to do different things when `Esc` is pressed and different
`Mode`s are active:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-mode.rs:app_handle_event}}
}
```

You'll need to add a new `switch_mode` method:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-mode.rs:app_switch_mode}}
}
```

## Draw

Let's make our view a little more interesting with some placeholder widgets.

### Results

For the results, use a `Table` with some mock data

```rust title="src/app.rs"
use itertools::Itertools;

impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-mode.rs:app_results_table_widget}}
}
```

### Prompt

For the prompt, make a `Block` that changes border color based on the mode:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-mode.rs:app_prompt_widget}}
}
```

### Render

And in the render function for the `StatefulWidget` we can call these widget constructors:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-mode.rs:app_statefulwidget}}
```

## Conclusion

If you run it, you should see something like this:

![](./crates-tui-tutorial-part-app-mode.gif)

Here's the full `./src/app.rs` file for your reference:

```rust collapsed title="src/app.rs (click to expand)"
use color_eyre::eyre::Result;
use itertools::Itertools;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::{
    events::{Event, Events},
    tui::Tui
};

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-mode.rs:full_app}}
```
