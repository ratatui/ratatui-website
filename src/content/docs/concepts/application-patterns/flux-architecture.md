---
title: Flux Architecture
sidebar:
  order: 3
---

[Flux](https://facebookarchive.github.io/flux/docs/in-depth-overview/) is a design pattern
introduced by Facebook to address the challenges of building large scale web applications. Though
originally designed with web applications in mind, the Flux architecture can be applied to any
client-side project, including terminal applications. Here's real world example of using the `Flux`
architecture with `ratatui`: <https://github.com/Yengas/rust-chat-server/tree/main/tui>.

## Why `Flux` for `ratatui`?

Terminal applications often have to deal with complex user interactions, multiple views, and dynamic
data sources. Keeping the application predictable and the logic decoupled is crucial. `Flux`, with
its unidirectional data flow, allows `ratatui` developers to have a structured way to handle user
input, process data, and update the views.

## `Flux` `ratatui` Overview

### Dispatcher

The dispatcher remains the central hub that manages all data flow in your application. Every action
in the application, whether it's a user input or a response from a server, will be channeled through
the dispatcher. This ensures a unified way of handling data, and since the dispatcher has no logic
of its own, it simply ensures that all registered callbacks receive the action data.

```rust
struct Dispatcher {
    store: Store,
}

impl Dispatcher {
    fn dispatch(&mut self, action: Action) {
        self.store.update(action);
    }
}
```

### Stores

Stores in Ratatui hold the application's state and its logic. They could represent things like:

- A list of items in a menu.
- The content of a text editor or viewer.
- User configurations or preferences.

Stores listen for actions dispatched from the Dispatcher. When a relevant action is dispatched, the
store updates its state and notifies any listening components (or views) that a change has occurred.

```rust
struct Store {
    counter: i32,
}

impl Store {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Increment => self.counter += 1,
            Action::Decrement => self.counter -= 1,
        }
    }

    fn get_state(&self) -> i32 {
        self.counter
    }
}

```

### Actions

Actions represent any change or event in your application. For instance, when a user presses a key,
selects a menu item, or inputs text, an action is created. This action is dispatched and processed
by the relevant stores, leading to potential changes in application state.

```rust
enum Action {
    Increment,
    Decrement,
}
```

### Views / Widgets

`ratatui`'s widgets display the application's UI. They don't hold or manage the application state,
but they display it. When a user interacts with a widget, it can create an action that gets
dispatched, which may lead to a change in a store, which in turn may lead to the widget being
updated.
