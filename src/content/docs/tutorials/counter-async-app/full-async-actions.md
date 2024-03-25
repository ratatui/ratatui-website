---
title: Full Async Actions
sidebar:
  order: 4
  label: Async Actions
---

Now that we have introduced `Event`s and `Action`s, we are going introduce a new `mpsc::channel` for
`Action`s. The advantage of this is that we can programmatically trigger updates to the state of the
app by sending `Action`s on the channel.

Here's the `run` function refactored from before to introduce an `Action` channel. In addition to
refactoring, we store the `action_tx` half of the channel in the `App`.

```rust
{{#include @code/ratatui-counter-async-app/src/main.rs:run}}
```

Running the code with this change should give the exact same behavior as before.

Now that we have stored the `action_tx` half of the channel in the `App`, we can use this to
schedule tasks. For example, let's say we wanted to press `J` and `K` to perform some network
request and _then_ increment the counter.

First, we have to update my `Action` enum:

```rust
{{#include @code/ratatui-counter-async-app/src/main.rs:action_enum}}
```

Next, we can update my event handler:

```rust
{{#include @code/ratatui-counter-async-app/src/main.rs:get_action}}
```

Finally, we can handle the action in my `update` function by spawning a tokio task:

```rust
{{#include @code/ratatui-counter-async-app/src/main.rs:update}}
```

Here is the full code for reference:

```rust
{{#include @code/ratatui-counter-async-app/src/main.rs:all}}
```

With that, we have a fully async application that is tokio ready to spawn tasks to do work
concurrently.
