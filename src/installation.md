# Installation

`ratatui` is a standard rust crate and can be installed into your app using the following command:

```shell
cargo add ratatui --features all-widgets
cargo add crossterm
```

or by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
crossterm = "0.27.0"
ratatui = { version = "0.23.0", features = ["all-widgets"]}
```

By default, Ratatui enables the Crossterm, but it's possible to alternatively use Termion, or
Termwiz instead by enabling the appropriate feature and disabling the default features. See
[Backend] for more information.

For Termion:

```shell
cargo add ratatui --no-default-features --features all-widgets,termion
cargo add termion
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.23", default-features = false, features = ["all-widgets", "termion"] }
termion = "2.0.1"
```

For Termwiz:

```shell
cargo add ratatui --no-default-features --features all-widgets,termwiz
cargo add termwiz
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.23", default-features = false, features = ["all-widgets", "termion"] }
termwiz = "0.20.0"
```

[Backend]: ./concepts/backends/README.md
