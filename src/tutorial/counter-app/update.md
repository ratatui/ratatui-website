# update.rs

Finally we have the `update.rs` file. Here, the `update()` function takes in two arguments:

- `key_event`: This is an event provided by the `crossterm` crate, representing a key press from the
  user.
- `app`: A mutable reference to our application's state, represented by the `App` struct.

```rust
{{#include ./ratatui-counter-app/src/update.rs}}
```

```admonish question
As an exercise, can you refactor this app to use "The Elm Architecture" principles?

Check out [the concepts page on The Elm Architecture](./../../concepts/the-elm-architecture.md) for reference.
```
