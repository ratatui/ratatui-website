---
title: Feature Flags
sidebar:
  order: 1
---

Feature flags let you choose a terminal backend, opt into integrations, and avoid compiling code
your application does not use. This page describes the flags exposed by Ratatui 0.30.2. The [docs.rs
feature graph] shows which features enable other features. Check the [feature definitions] in
Ratatui's source when using another release.

## Default Features

A plain `ratatui` dependency enables these features:

| Feature           | What it enables                                                 |
| ----------------- | --------------------------------------------------------------- |
| `all-widgets`     | Every dependency-gated widget, currently only `widget-calendar` |
| `crossterm`       | The Crossterm backend, using Crossterm 0.29                     |
| `layout-cache`    | Caching for repeated layout calculations                        |
| `macros`          | [Macros] for constructing spans, lines, text, rows, and layouts |
| `underline-color` | [`Style::underline_color`] on supported backends and platforms  |

Use `default-features = false` when you need to replace this set rather than add to it. Features are
additive: disabling defaults means that you must explicitly enable every feature you need.

## Backend Selection

For most cases, the default `crossterm` backend is the correct choice. See
[Backends](/concepts/backends/) for more information. However, this can be changed to termion,
termwiz, or termina.

| Feature          | Default | What it selects                                       |
| ---------------- | ------- | ----------------------------------------------------- |
| `crossterm`      | Yes     | Crossterm and [`CrosstermBackend`]                    |
| `crossterm_0_28` | No      | Crossterm 0.28 support; read the version notes below  |
| `crossterm_0_29` | No      | Crossterm 0.29 support, also selected by `crossterm`  |
| `termina`        | No      | Termina and [`TerminaBackend`]                        |
| `termion`        | No      | Termion and [`TermionBackend`] on non-Windows targets |
| `termwiz`        | No      | Termwiz and [`TermwizBackend`]                        |

Ratatui re-exports the backend crate selected by each feature. This helps libraries refer to the
backend version Ratatui uses. Applications should generally add the backend as a direct dependency
and import it directly, such as `crossterm::event`. This makes each type's source clear and lets the
application select backend-specific features.

```shell
# Defaults to crossterm
cargo add ratatui crossterm

# For termion, unset the default crossterm feature and select the termion feature
cargo add ratatui --no-default-features --features=termion
cargo add termion

# For termwiz, unset the default crossterm feature and select the termwiz feature
cargo add ratatui --no-default-features --features=termwiz
cargo add termwiz

# For termina, unset the default crossterm feature and select the termina feature
cargo add ratatui --no-default-features --features=termina
cargo add termina
```

### Crossterm Versions

Ratatui 0.30.2's default `crossterm` feature selects Crossterm 0.29, so most applications should
use:

```shell
cargo add ratatui crossterm
```

The version-specific features belong to the split [`ratatui-crossterm`] package. Although the main
`ratatui` package forwards those features, enabling `crossterm_0_28` there also activates
`ratatui-crossterm`'s default Crossterm 0.29 feature. Both versions are compiled, and the backend
uses 0.29 because the newer enabled version takes precedence.

Applications that must use Crossterm 0.28 should select it on `ratatui-crossterm` directly:

```toml
[dependencies]
crossterm = "0.28"
ratatui = { version = "0.30.2", default-features = false }
ratatui-crossterm = { version = "0.1.2", default-features = false, features = ["crossterm_0_28"] }
```

This uses the backend through `ratatui_crossterm::CrosstermBackend`, rather than the backend and
setup helpers re-exported by the main package. Enable any other Ratatui features the application
needs explicitly.

Do not enable both version-specific features. If both are enabled, the backend uses the newer
version. A direct dependency on the older version would then create two distinct Crossterm types and
two copies of its terminal state.

Use `cargo tree -p crossterm` to see which versions Cargo resolved. The [backend compatibility
guide] describes the compilation errors, missed events, and terminal-restoration problems caused by
mismatched versions.

[backend compatibility guide]: /concepts/backends/#crossterm-version-compatibility

## Core and Integration Features

These features work independently of the selected backend unless the description says otherwise:

| Feature             | Default | What it enables                                            |
| ------------------- | ------- | ---------------------------------------------------------- |
| `std`               | Yes[^1] | Standard-library support                                   |
| `serde`             | No      | Serialization for [style and color types]                  |
| `layout-cache`      | Yes     | An LRU cache configurable with [`Layout::init_cache`]      |
| `macros`            | Yes     | [Macros] for spans, lines, text, rows, and layouts         |
| `palette`           | No      | Conversions from [`palette`] colors to Ratatui's [`Color`] |
| `portable-atomic`   | No      | [`portable-atomic`] on targets without native atomics      |
| `scrolling-regions` | No      | Scrolling regions used by [`Terminal::insert_before`]      |
| `underline-color`   | Yes     | [`Style::underline_color`] on supported terminals          |

[^1]: Each backend feature enables `std`, including the default `crossterm` backend.

`scrolling-regions` reduces flicker when [`Terminal::insert_before`] inserts content above an inline
viewport. `underline-color` works with Crossterm, Termina, and Termwiz, but not on Windows 7.

Enable an additional feature without changing the defaults:

```shell
cargo add ratatui --features serde
```

Ratatui supports `no_std` builds. Disable the default features and do not select a backend to leave
`std` disabled. Most terminal applications need `std`; this setup is primarily useful for libraries
and custom integrations.

## Widget Features

Widgets that introduce another dependency have their own feature flags. `all-widgets` enables all of
them. In Ratatui 0.30.2, the only such widget is the calendar:

| Feature           | Default | What it enables                                    |
| ----------------- | ------- | -------------------------------------------------- |
| `all-widgets`     | Yes     | Every dependency-gated widget                      |
| `widget-calendar` | Yes[^2] | The [calendar widget] and its dependency on `time` |

[^2]: Enabled through `all-widgets`.

:::note

`all-widgets` is enabled by default. Disable the defaults and enable `widget-calendar` directly if
you want the calendar without the other default features.

:::

```shell
cargo add ratatui --no-default-features --features=all-widgets
```

```shell
cargo add ratatui --no-default-features --features=widget-calendar
```

## Unstable Features

Unstable features expose experimental APIs that may change in any release. Enable the narrowest flag
that provides the API you need instead of enabling the aggregate `unstable` flag.

| Feature                       | What it enables                                             |
| ----------------------------- | ----------------------------------------------------------- |
| `unstable`                    | All unstable features                                       |
| `unstable-backend-writer`     | Backend [`writer()`] and [`writer_mut()`] accessors         |
| `unstable-rendered-line-info` | [`Paragraph::line_count()`] and [`Paragraph::line_width()`] |
| `unstable-widget-ref`         | The [`WidgetRef`] and [`StatefulWidgetRef`] traits          |

For example, enable only the rendered-line APIs with:

```shell
cargo add ratatui --features unstable-rendered-line-info
```

[calendar widget]: https://docs.rs/ratatui/latest/ratatui/widgets/calendar/
[docs.rs feature graph]: https://docs.rs/crate/ratatui/latest/features
[feature definitions]:
  https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui/Cargo.toml#L23-L115
[Macros]: https://docs.rs/ratatui-macros/latest/ratatui_macros/
[style and color types]: https://docs.rs/ratatui/latest/ratatui/style/
[`Color`]: https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html
[`CrosstermBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html
[`Layout::init_cache`]:
  https://docs.rs/ratatui/latest/ratatui/layout/struct.Layout.html#method.init_cache
[`palette`]: https://docs.rs/palette/latest/palette/
[`Paragraph::line_count()`]:
  https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html#method.line_count
[`Paragraph::line_width()`]:
  https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html#method.line_width
[`portable-atomic`]: https://docs.rs/portable-atomic/latest/portable_atomic/
[`ratatui-crossterm`]: https://docs.rs/crate/ratatui-crossterm/latest/features
[`StatefulWidgetRef`]: https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidgetRef.html
[`Style::underline_color`]:
  https://docs.rs/ratatui/latest/ratatui/style/struct.Style.html#method.underline_color
[`Terminal::insert_before`]:
  https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.insert_before
[`TerminaBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TerminaBackend.html
[`TermionBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TermionBackend.html
[`TermwizBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TermwizBackend.html
[`WidgetRef`]: https://docs.rs/ratatui/latest/ratatui/widgets/trait.WidgetRef.html
[`writer()`]:
  https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html#method.writer
[`writer_mut()`]:
  https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html#method.writer_mut
