---
title: App Prototype
---

In this section, we are going to expand on the `App` struct to add channels and actions.

## Actions

One of the first steps to building truly `async` TUI applications is to use the `Command`, `Action`,
or `Message` pattern.

:::tip

The `Command` pattern is a behavioral design pattern that represents function call as a stand-alone
object that contains all information about the function call.

You can learn more from:

- https://refactoring.guru/design-patterns/command
- http://gameprogrammingpatterns.com/command.html
- [The Elm Architecture section](/concepts/application-patterns/the-elm-architecture/)

:::

The key idea here is that `Action` enum variants maps exactly to different methods on the `App`
struct, and the variants of `Action` represent all the actions that can be carried out by an `app`
instance to update its state.

The variants of the `Action` enum you will be using for this tutorial are:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:action}}
```

## Channels

Define the following fields in the `App` struct:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:app}}
```

where `tx` and `rx` are two parts of the pair of the `Action` channel from `tokio::mpsc`, i.e.

- `tx`: Transmitter
- `rx`: Receiver

These pairs are created using the `tokio::mpsc` channel, which stands for multiple producer single
consumer channels. These pairs from the channel can be used sending and receiving `Action`s across
thread and task boundaries.

Practically, what this means for your application is that you can pass around clones of the
transmitter to any children of the `App` struct and children can send `Action`s at any point in the
operation of the app to trigger a state change in `App`. This works because you have a single `rx`
here in the root `App` struct that receives those `Action`s and acts on them.

This allows you as a Ratatui app developer to organize your application in any way you please, and
still propagate information up from child to parent structs using the `tx` transmitter.

Setup a `App::new()` function to construct an `App` instance like so:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:app_new}}
}
```

Let's also update the `async run` method now:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:app_run}}
```

## handle_event

Update `handle_event` to delegate to `Mode` to figure out which `Action` should be generated based
on the key event and the `Mode`.

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:app_handle_event}}
}
```

Most of the work in deciding which `Action` should be taken is done in `Mode::handle_key`. Since
this is oriented around `Mode`, implement the `handle_key` method on `Mode` in the following manner:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:mode}}
```

:::note

If the `maybe_action` is a `Some` variant, it is sent over the `tx` channel:

```rust
        maybe_action.map(|action| self.tx.send(action));
```

:::

## handle_action

Now implement the `handle_action` method like so:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:app_handle_action}}
}
```

Because the `run` method has the following block of code, any `Action` received on `rx` will trigger
an call to the `handle_action` method.

```rust
while let Ok(action) = self.rx.try_recv() {
    self.handle_action(action.clone(), &mut tui)?;
}
```

Since this is a `while let` loop, multiple `Action`s can be queued in your application and the
`while let` will only return control back to the `run` method when all the actions have been
processed.

Any time the `rx` receiver receives an `Action` from _any_ `tx` transmitter, the application will
"handle the action" and the state of the application will update. This means you can, for example,
send a new variant `Action::Error(String)` from deep down in a nested child instance, which can
force the app to show an error message as a popup. You can also pass a clone of the `tx` into a
tokio task, and have the tokio task propagate information back to the `App` asynchronously. This is
particularly useful for error messages when a `.unwrap()` would normally fail in a tokio task.

While introducing `Action`s in between `Event`s and the app methods may seem like a lot more
boilerplate at first, using an `Action` enum this way has a few advantages.

Firstly, `Action`s can be mapped from keypresses in a declarative manner. For example, you can
define a configuration file that reads which keys are mapped to which `Action` like so:

```toml
[keyconfig]
"q" = "Quit"
"j" = "ScrollDown"
"k" = "ScrollUp"
```

Then you can add a new `keyconfig` in the `App` like so:

```rust
struct App {
  ...
  // new field
  keyconfig: HashMap<KeyCode, Action>
}
```

If you populate `keyconfig` with the contents of a user provided `toml` file, then you can figure
out which action to take directly from the keyconfig struct:

```rust
fn handle_event(&mut self, event: Event) -> Option<Action> {
  if let Event::Key(key) = event {
    return self.keyconfig.get(key.code)
  };
  None
}
```

A second reason you may want to use `Action`s is that it allows us to send a `tx` into a long
running task and retrieve information back from the task during its execution. For example, if a
task errors, you can send an `Action::Error(String)` back to the app which can then be displayed as
a popup.

For example, you can send an `Action::UpdateSearchResults` from inside the task once the query is
complete, when can make sure that the first time is selected after the results are loaded (by
scrolling down):

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:app_submit_search_query}}

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:app_update_search_results}}
}
```

Finally, using an `Action` even allows us as app developers to trigger an action from anywhere in
any child struct by sending an `Action` over `tx`.

Here's the full `./src/app.rs` file for your reference:

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

{{#include @code/crates-tui-tutorial-app/src/bin/part-app-prototype.rs:full_app}}
```

## Conclusion

This is what our app currently looks like:

![](./crates-tui-tutorial-part-app-prototype.gif)

However, currently everything is in a single file, and the `App` struct is starting to get a little
unwieldy. If we want to add more features or more widgets, this approach isn't going to scale very
well.

In the rest of the tutorial, we are going to refactor the app into `StatefulWidget`s and add more
polish.

Your folder structure should currently look like this:

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
