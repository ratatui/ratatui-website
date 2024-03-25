---
title: How to use `color_eyre` with Ratatui
sidebar:
  order: 6
  label: color_eyre Error Hooks
---

Full source code for this how to article is available at:
<https://github.com/ratatui-org/ratatui-website/tree/main/code/how-to-color_eyre/>

The [`color_eyre`] crate provides error report handlers for panics and errors. It displays the
reports formatted and in color. To use these handlers, a Ratatui app needs to restore the terminal
before displaying the errors.

## Installation

First add the crate to your `Cargo.toml`

```shell title="add color_eyre to Cargo.toml"
cargo add color_eyre
```

Add the following imports to `main.rs`

```rust
// main.rs
{{ #include @code/how-to-color_eyre/src/main.rs:imports }}
```

Create a new function `install_hooks()` which will ensure your app calls `tui::restore()` before
exiting with a panic or an error.

```rust
// main.rs
{{ #include @code/how-to-color_eyre/src/main.rs:install_hooks }}
```

This example assumes that you have a `tui` module in your app with `init` and `restore` functions

<details><summary>Example tui.rs module</summary>

```rust
// tui.rs
{{ #include @code/how-to-color_eyre/src/tui.rs }}
```

</details>

## Usage

In your application, wrap errors with extra context as needed:

Add the following import:

```rust
// main.rs
{{ #include @code/how-to-color_eyre/src/main.rs:error_imports}}
```

Call wrap_err from methods that can fail with an error.

```rust
// main.rs
{{ #include @code/how-to-color_eyre/src/main.rs:main}}
```

## Demo

<details><summary>Full code</summary>

```rust
// main.rs
{{ #include @code/how-to-color_eyre/src/main.rs }}
```

```rust
// tui.rs
{{ #include @code/how-to-color_eyre/src/tui.rs }}
```

</details>

### Panic

![Panic](./color-eyre/panic.png)

With `RUST_BACKTRACE=full`:

![Panic Full](./color-eyre/panic-full.png)

### Error

![Error](./color-eyre/error.png)

With `RUST_BACKTRACE=full`:

![Error Full](./color-eyre/error-full.png)

### Normal exit

![Quit](./color-eyre/quit.png)

## Further Steps

See the `color_eyre` [docs] and [examples] for more advanced setups. E.g.:

- [Capturing span traces](https://github.com/eyre-rs/color-eyre/blob/master/examples/usage.rs)
- [Configuring an automatic issue url](https://github.com/eyre-rs/color-eyre/blob/master/examples/github_issue.rs)

[`color_eyre`]: https://crates.io/crates/color-eyre
[docs]: https://github.com/eyre-rs/color-eyre/blob/master/examples/
[examples]: https://github.com/eyre-rs/color-eyre/blob/master/examples/
