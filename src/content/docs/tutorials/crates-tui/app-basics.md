---
title: App
---

Before we proceed any further, we are going to refactor the code we already have to make it easier
to scale up. We are going to move the event loop into a method on the `App` struct.

Create a new file `./src/app.rs`:

```rust
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:app}}
```

Define some helper functions for initializing the `App`:

```rust
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:app_new}}
}

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:app_default}}
```

Now define a `run` method for `App`:

```rust
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:app_run}}
}
```

:::note

This run method is `async` and uses `events.next().await`, which returns a `Event` from the stream
you created earlier.

:::

The `run` method uses a `should_quit` method (and a corresponding `quit` method) that you can define
like this:

```rust
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:app_quit}}
}
```

This `run` method also uses a `handle_event` method that you can define like so:

```rust
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:app_handle_event}}
}
```

Finally, for the `draw` method, you could define it like this:

```rust
use ratatui::widgets::*;

impl App {
    fn draw(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            frame.render_widget(
                Paragraph::new(format!(
                    "frame counter: {}",
                    frame.count()
                )),
                frame.size(),
            );
        })?;
        Ok(())
    }
}
```

But let's go one step further and set ourselves up for using the `StatefulWidget` pattern.

Define the `draw` method like this:

```rust
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:app_draw}}
}
```

This uses a [unit-like struct] called `AppWidget` that can be rendered as a `StatefulWidget` using
the `App` struct as its state.

[unit-like struct]:
  https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields

```rust
use ratatui::widgets::{StatefulWidget, Paragraph};

struct AppWidget;

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:app_statefulwidget}}
```

Here's the full `./src/app.rs` file for your reference:

<details>

<summary>Copy the following into <code>src/app.rs</code></summary>

```rust
use color_eyre::eyre::Result;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::{
    events::{Event, Events},
    tui::Tui
};

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-basics.rs:full_app}}
```

</details>

Now, run your application with a modified `main.rs` that uses the `App` struct you just created:

```rust
pub mod app;
pub mod errors;
pub mod events;
pub mod tui;
pub mod widgets;

{{#include @code/crates-tui-tutorial-app/src/bin/part-final.rs:main}}
```

You should get the same results as before.

![](./crates-tui-tutorial-part-app-basics.gif)

Your file structure should now look like this:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
   ├── app.rs
   ├── crates_io_api_helper.rs
   ├── errors.rs
   ├── events.rs
   ├── main.rs
   └── tui.rs
```
