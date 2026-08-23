---
title: Installation
sidebar:
  order: 0
---

[Ratatui 0.30.2] requires Rust 1.88 or newer. Install the current stable Rust toolchain with
[`rustup`](https://rustup.rs/), then add Ratatui and its default [Crossterm] backend to a Cargo
project:

```shell
cargo add ratatui crossterm
```

The command adds both dependencies to `Cargo.toml`:

```toml
[dependencies]
crossterm = "0.29"
ratatui = "0.30.2"
```

Ratatui 0.30.2 uses Crossterm 0.29 by default, so these versions match. Keep the direct Crossterm
dependency on the same major version Ratatui selects. Cargo can compile two incompatible Crossterm
versions into one application, but their event types and terminal state are separate. See
[Crossterm versions](/installation/feature-flags/#crossterm-versions) before selecting an older
version.

By default, `ratatui` enables the `crossterm` feature, but it's possible to alternatively use
`termion`, `termwiz`, or `termina` instead by enabling the appropriate feature and disabling the
default features. See [Backend] for more information.

:::note

Ratatui re-exports its selected backend crate, such as [`ratatui::crossterm`]. This helps library
code refer to the backend version Ratatui uses. For applications, a direct backend dependency and
imports such as [`crossterm::event`] usually make the source clearer and allow backend-specific
features to be selected directly.

:::

For [Termion]:

```shell
cargo add ratatui --no-default-features --features termion
cargo add termion
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.30.2", default-features = false, features = ["termion"] }
termion = "4"
```

For [Termwiz]:

```shell
cargo add ratatui --no-default-features --features termwiz
cargo add termwiz
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.30.2", default-features = false, features = ["termwiz"] }
termwiz = "0.23"
```

For [Termina]:

```shell
cargo add ratatui --no-default-features --features termina
cargo add termina
```

or in your `Cargo.toml`:

```toml
[dependencies]
ratatui = { version = "0.30.2", default-features = false, features = ["termina"] }
termina = "0.3"
```

[Backend]: /concepts/backends/
[Crossterm]: https://docs.rs/crossterm/latest/crossterm/
[Ratatui 0.30.2]: https://docs.rs/ratatui/0.30.2/ratatui/
[Termina]: https://docs.rs/termina/latest/termina/
[Termion]: https://docs.rs/termion/latest/termion/
[Termwiz]: https://docs.rs/termwiz/latest/termwiz/
[`crossterm::event`]: https://docs.rs/crossterm/latest/crossterm/event/
[`ratatui::crossterm`]: https://docs.rs/ratatui/latest/ratatui/#reexports
