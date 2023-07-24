# Installation

Installing ratatui is as simple as
```console
cargo add ratatui
```
and you can start programming TUIs.

However, there are a few features that are worth a look.

## Optional Features

As ratatui grows and evolves, this list may change, so make sure to check the [main repo](./https://github.com/ratatui-org/ratatui) if you are unsure.

### Feature: Backend Selection

See [Choosing a Backend](./ch01-02-choosing-a-backend.md) for more information. However, for most cases, the default `crossterm` backend is enough.

```console
# Defaults to crossterm
cargo add ratatui
# Unset the default crossterm feature and select one of the other backends
cargo add ratatui --no-default-features --features=terminon 
cargo add ratatui --no-default-features --features=termwiz
```

### Feature: All-Widgets

This feature enables some extra widgets that are not in `default` to save on compile time. As of v0.21, the only widget in this feature group is the `calendar` widget, which can be enabled with the `widget-calendar` feature.
```console
cargo add ratatui --features all-widgets
```

### Feature: Widget-Calendar
This feature enables the calendar widget, which requires the `time` crate. 

```console
cargo add ratatui --features widget-calendar
```

### Feature: Serde

```console
cargo add ratatui --features serde
```
