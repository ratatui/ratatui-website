# Full Async - `Action`s

Now that we have introduced `Event`s and `Action`s, we can introduce a channel for `Action`s.

Here's the `run` function refactored from before to introduce an `Action` channel. In addition to
refactoring, we store the `action_tx` half of the channel in the `App`.

```rust
{{#include ./ratatui-counter-async-app/src/main.rs:run}}
```

Now that we have stored the `action_tx`, we can use this to schedule async work. For example, let's
say we wanted to press `J` and `K` to perform some network request and _then_ increment the counter.

First, we have to update my `Action` enum:

```rust
{{#include ./ratatui-counter-async-app/src/main.rs:action_enum}}
```

Next, we can update my event handler:

```rust
{{#include ./ratatui-counter-async-app/src/main.rs:get_action}}
```

Finally, we can handle the action in my `update` function my spawning a tokio task:

```rust
{{#include ./ratatui-counter-async-app/src/main.rs:update}}
```

Here is the full code for reference (with an improved UI to boot):

```rust
{{#include ./ratatui-counter-async-app/src/main.rs:all}}
```

With that, we have a fully async application that is tokio ready to spawn tasks to do work
concurrently.
