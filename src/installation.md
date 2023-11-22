# Installation

`ratatui` is a standard rust crate and can be installed into your app using the following command:

```shell
cargo add ratatui crossterm
```

or by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
crossterm = "0.27.0"
ratatui = "0.24.0"
```

````admonish tip

Additionally, you can use the `all-widgets` feature, which enables additional widgets:

```shell
cargo add ratatui --features all-widgets
cargo add crossterm
```

or by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
crossterm = "0.27.0"
ratatui = { version = "0.24.0", features = ["all-widgets"]}
```

You can learn more about available widgets from the
[docs.rs page on widgets](https://docs.rs/ratatui/latest/ratatui/widgets/index.html).

````

By default, `ratatui` enables the `crossterm`, but it's possible to alternatively use `termion`, or
`termwiz` instead by enabling the appropriate feature and disabling the default features. See
[Backend] for more information.

For Termion:

```shell
cargo add ratatui --no-default-features --features termion
cargo add termion
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.23", default-features = false, features = ["termion"] }
termion = "2.0.1"
```

For Termwiz:

```shell
cargo add ratatui --no-default-features --features termwiz
cargo add termwiz
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.23", default-features = false, features = ["termwiz"] }
termwiz = "0.20.0"
```

[Backend]: ./concepts/backends/
