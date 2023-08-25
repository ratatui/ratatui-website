# handler.rs

Finally we have the `handler.rs` file. Here, the `handle_key_events` function takes in two
arguments:

- `key_event`: This is an event provided by the `crossterm` crate, representing a key press from the
  user.
- `app`: A mutable reference to our application's state, represented by the `App` struct.

```rust
{{#include ./ratatui-counter-app/src/handler.rs}}
```
