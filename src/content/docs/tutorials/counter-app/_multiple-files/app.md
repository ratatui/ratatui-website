---
title: app.rs
---

Let's start with the same `struct` as we had before:

```rust
{{#include @code/tutorials/ratatui-counter-app/src/app.rs:application}}
```

We can add additional methods to this `Application` struct:

```rust
{{#include @code/tutorials/ratatui-counter-app/src/app.rs:application_impl}}
```

We use the principle of encapsulation to expose an interface to modify the state. In this particular
instance, it may seem like overkill but it is good practice nonetheless.

The practical advantage of this is that it makes the state changes easy to test.

```rust
{{#include @code/tutorials/ratatui-counter-app/src/app.rs:application_test}}
```

:::tip

You can test a single function by writing out fully qualified module path to the test function, like
so:

```bash
cargo test -- app::tests::test_app_increment_counter --nocapture
```

Or even test all functions that start with `test_app_` by doing this:

```bash
cargo test -- app::tests::test_app_ --nocapture
```

The `--nocapture` flag prints stdout and stderr to the console, which can help debugging tests.

:::
