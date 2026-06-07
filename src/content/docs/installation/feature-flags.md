---
title: Feature Flags
sidebar:
  order: 1
---

As ratatui grows and evolves, this list may change, so make sure to check the
[main repo](https://github.com/ratatui/ratatui) if you are unsure.

## Backend Selection

For most cases, the default `crossterm` backend is the correct choice. See
[Backends](/concepts/backends/) for more information. However, this can be changed to termion or
termwiz

```shell
# Defaults to crossterm
cargo add ratatui

# For termion, unset the default crossterm feature and select the termion feature
cargo add ratatui --no-default-features --features=termion
cargo add termion

# For termwiz, unset the default crossterm feature and select the termwiz feature
cargo add ratatui --no-default-features --features=termwiz
cargo add termwiz
```

## All-Widgets

As of v0.21, the only widget in this feature group is the `calendar` widget, which can be enabled
with the `widget-calendar` feature.

:::note

Starting from v0.30.0, the `all-widgets` feature is enabled by default. It can be disabled to save
on compile time.

:::

```shell
cargo add ratatui --no-default-features --features=all-widgets
```

## Widget-Calendar

This feature enables the calendar widget, which requires the `time` crate. It is enabled by default
as part of the `all-widgets` feature.

```shell
cargo add ratatui --no-default-features --features=widget-calendar
```

## Serde

Enables serialization and deserialization of style and color types using the Serde crate. This is
useful if you want to save themes to a file.

```shell
cargo add ratatui --features serde
```
