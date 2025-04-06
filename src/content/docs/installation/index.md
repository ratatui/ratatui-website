---
title: Installation
sidebar:
  order: 0
---

`ratatui` is a standard rust crate and can be installed into your app using the following command:

```shell
cargo add ratatui
```

or by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
ratatui = "0.28.0"
```

:::tip

Additionally, you can use the `all-widgets` feature, which enables additional widgets:

```shell
cargo add ratatui --features all-widgets
```

or by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
ratatui = { version = "0.28.0", features = ["all-widgets"] }
```

You can learn more about available widgets from the
[docs.rs page on widgets](https://docs.rs/ratatui/latest/ratatui/widgets/index.html).

:::

By default, `ratatui` enables the `crossterm` feature, but it's possible to alternatively use
`termion`, or `termwiz` instead by enabling the appropriate feature and disabling the default
features. See [Backend] for more information.

:::note

Before Ratatui 0.27.0, it was necessary to import a backend crate that matched the backend feature.
In 0.27.0 Ratatui now exports the backend crates at the root to make it easier to ensure a matching
version of the backend crate is used.

:::

For Termion:

```shell
cargo add ratatui --no-default-features --features termion
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.28.0", default-features = false, features = ["termion"] }
```

For Termwiz:

```shell
cargo add ratatui --no-default-features --features termwiz
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.28.0", default-features = false, features = ["termwiz"] }
```

[Backend]: /concepts/backends/
