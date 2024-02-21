---
title: Main
---

Let's make the `main` function a `tokio` entry point.

Add the `#[tokio::main]` macro to the `main` function and make the function `async`. This allows you
to use `async` and `await` inside `main`. You can also now spawn tokio tasks within your
application.

```rust title="src/main.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-main.rs}}
```

You can run this with `cargo run`, and you'll see that the terminal prints and then blocks for 5
seconds before returning control.

```bash
$ cargo run
   Compiling crates-tui v0.1.0 (~/gitrepos/crates-tui-tutorial)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/crates-tui-tutorial`
Sleeping for 5 seconds...
$
```

:::tip

On UNIX systems, you can use `time cargo run` to see how long a process takes to run.

<!--
time cargo run --bin part-main
-->

```bash
$ time cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/crates-tui-tutorial`
Sleeping for 5 seconds...
cargo run  0.09s user 0.05s system 2% cpu 5.262 total
$
```

In this case, it took `5.262` seconds to run `cargo run`.

:::

:::note[Homework]

Try to predicate what happens if you spawn multiple tokio tasks? e.g.

```rust title="src/main.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-main-tasks-concurrent.rs}}
```

<!--
Is this good for a user to think about?

In the above example, if you make this change:

```diff
- #[tokio::main]
+ #[tokio::main(flavor = "current_thread")]
```

can you predict how the code will behave differently? Run it to confirm. Do you understand why it
behaves the way it does?
-->

Now, what happens if you run the following instead?

```rust title="src/main.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-main-tasks-sequential.rs}}
```

Do you understand the different between creating a future and `await`ing on it later _versus_
spawning a future and `await`ing on the spawn's `JoinHandle` later?

:::

<!--

```
$ time cargo run --bin part-main-tasks-concurrent

Spawning a task that sleeps 5 seconds...
Getting return values from tasks...
Sleeping for 5 seconds in a tokio task 0...
Sleeping for 5 seconds in a tokio task 2...
Sleeping for 5 seconds in a tokio task 1...
Sleeping for 5 seconds in a tokio task 3...
Sleeping for 5 seconds in a tokio task 5...
Sleeping for 5 seconds in a tokio task 6...
Sleeping for 5 seconds in a tokio task 4...
Sleeping for 5 seconds in a tokio task 8...
Sleeping for 5 seconds in a tokio task 7...
Sleeping for 5 seconds in a tokio task 9...
Got i = 9
Got i = 8
Got i = 7
Got i = 6
Got i = 5
Got i = 4
Got i = 3
Got i = 2
Got i = 1
Got i = 0

cargo run --bin part-main-tasks-concurrent  0.09s user 0.05s system 2% cpu 5.385 total
```

When spawning tasks it only takes 5 seconds.

```
$ time cargo run --bin part-main-tasks-sequential

Spawning a task that sleeps 5 seconds...
Getting return values from tasks...
Sleeping for 5 seconds in a tokio task 9...
Got i = 9
Sleeping for 5 seconds in a tokio task 8...
Got i = 8
Sleeping for 5 seconds in a tokio task 7...
Got i = 7
Sleeping for 5 seconds in a tokio task 6...
Got i = 6
Sleeping for 5 seconds in a tokio task 5...
Got i = 5
Sleeping for 5 seconds in a tokio task 4...
Got i = 4
Sleeping for 5 seconds in a tokio task 3...
Got i = 3
Sleeping for 5 seconds in a tokio task 2...
Got i = 2
Sleeping for 5 seconds in a tokio task 1...
Got i = 1
Sleeping for 5 seconds in a tokio task 0...
Got i = 0

cargo run --bin part-main-tasks-sequential  0.10s user 0.05s system 0% cpu 50.520 total
```

Without a spawn, you are creating a future which is only polled when the future is awaited on.
And when it is awaited on, it blocks the current thread.

-->

We will expand on `main.rs` in the following sections. Right now, your project should look like
this:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
   └── main.rs
```
