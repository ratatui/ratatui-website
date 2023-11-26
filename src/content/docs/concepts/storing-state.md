---
title: Storing Application State
---

# Storing Application State

This page covers several ways that a programmer can store the state of the application.

## Single Silo Method

This is perhaps the easiest method to understand, and works best for small applications that do no
require a large amount of state to be remembered. The idea behind this method is simple: "One struct
for all state", and whenever a component requires knowledge about the state of the application, it
requests a reference to the `app` state.

This is the method used in the tutorial.

### Pros

This is conceptually very easy to understand. All of your states are stored in one place, and
passing it to sub-components is simple.

### Cons

However, you can tell when your application has outgrown the single silo application state when you
begin to write code like this:

```rust
        let selected_item = &app.states.history.transacts_list.items[app.states.history.transacts_list.state.selected().unwrap()];
```

Another downside to this method, is the lack of multithreaded support. If you begin to use multiple
threads that need access to the application state, access to the `app` can become a bottleneck as
`Mutex` and locks get handed around.
