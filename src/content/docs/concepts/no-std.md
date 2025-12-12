---
title: Using no_std
description: Running Ratatui without the Rust standard library and authoring no_std widgets.
---

:::note

`no_std` means is Rust _without_ the standard library.

In this mode your code can only use the `core` and `alloc` crates, so there is no operating system
integration, file I/O, threads, or heap allocator unless you provide one.

:::

Ratatui supports `no_std`, which allows it to run in embedded and other resource-constrained
environments. This means you can run TUIs on a wider range of targets and have widget usable in both
desktop and embedded environments.

## Using Ratatui without `std`

1. Disable default features so the crate does not pull in `std`-only dependencies:

```toml
ratatui = { version = "0.30", default-features = false }
```

:::note

You can re-enable only the features you need that are compatible with `no_std`. (e.g. `macros`,
`all-widgets`, etc.)

Skip features that explicitly require `std` such as `crossterm`, `serde` and so on.

:::

:::caution

On targets without atomic instructions, you must also enable `portable-atomic` to provide software
atomics:

```toml
ratatui = { version = "0.30", default-features = false, features = ["portable-atomic"] }
```

:::

2. Choose a backend that works on your platform.

The built-in backends rely on `std`, so `no_std` targets need a custom backend that implements
`ratatui::backend::Backend` or a third-party option like [`mousefood`](/ecosystem/mousefood/) 🧀

3. When checking a `no_std` build, compile with a `no_std` target.

For example, on ESP32:

```bash
cargo check --target riscv32imc-unknown-none-elf
```

## Creating `no_std` widgets

If you already have a Ratatui widget, you can make it `no_std`-compatible with a few small changes.
Even if you haven't built for embedded before!

1. Opt into `no_std` and expose `alloc` types:

   ```rust
   // lib.rs
   #![no_std]
   extern crate alloc;
   ```

2. Depend on `ratatui-core` instead of the full `ratatui` crate to avoid backend dependencies:

   ```toml
   ratatui-core = { version = "0.30", default-features = false }
   ```

3. Swap `std` types for their `core`/`alloc` equivalents, for example `core::fmt`,
   `alloc::string::String`, `alloc::vec::Vec`, and `alloc::boxed::Box`.

4. Keep a `std` feature (off by default) for conveniences like tests or examples, but write your
   core widget logic so it also works without it.

A minimal `no_std` widget implementation:

```rust
#![no_std]
extern crate alloc;

use alloc::string::String;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::text::Line;
use ratatui_core::widgets::Widget;

struct Greeting {
    message: String,
}

impl Widget for &Greeting {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::raw(&self.message).render(area, buf);
    }
}
```

:::note

Some tips for testing and maintaining `no_std` compatibility:

- Run `cargo check --no-default-features` (optionally with a `no_std` target) to catch regressions.
- Document which optional features are `no_std`-compatible so users know what to enable.

:::
