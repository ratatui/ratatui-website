---
title: Configurable Keybindings
sidebar:
  order: 11
  label: Configurable Vim Keybind Config
---

# Recipe: Configurable Keybindings in Ratatui Apps

This recipe explores how to add customizable, user-driven keybindings to your Ratatui application.
It covers common approaches for managing keybindings, supporting user configuration, and maintaining
backward compatibility as your application evolves. One concrete implementation using the
[`crossterm-keybind`](https://github.com/yanganto/crossterm-keybind) crate is presented as an
example.

## Problem statement and motivation

With growing userbases, developers of Terminal UI (TUI) apps often get requests for alternative
keybinding schemes (like vim-style bindings or personalized shortcuts). Manually supporting such
requests quickly becomes a maintenance burden, and as your app evolves, users expect their custom
keybinds to remain compatible across updates.

## Design and Constraints

### Core Pattern

The main idea is to define all keybindings in _a single enum_, use attribute macros to declare
default shortcuts, and support external TOML configuration for overrides and patches.

**The Enum Example**:

```rust
use crossterm_keybind::KeyBind;

#[derive(KeyBind)]
pub enum KeyEvent {
    /// Close the application
    #[keybindings["Control+c", "Q", "q"]]
    Quit,

    /// Toggle to open/close a widget show all the commands
    #[keybindings["h", "F1"]]
    ShowHelp,
}
```

#### How to capture a user input

Within an abstraction, the enum, you don't want to directly compare the `KeyCode`, `KeyModifiers` of
a `crossterm::KeyEvent` when capturing a user's input. Instead, you can pass a reference of it to a
`match_any` method, which can be provided by the KeyBind derive macro. Before you match any
keyevent, you should initialize first with `KeyEvent::init_and_load(...)`, because it is possible
for your users to have customized keybinds (this will be explained in the next section). It can be
initialized without any user customized keybind by `KeyEvent::init_and_load(None)`. Normally, you
can run it as the first task of the main function.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    KeyEvent::init_and_load(None)?;
}
```

```rust
if KeyBindEvent::Quit.match_any(&key) {
  // Close the app
} else if KeyBindEvent::ShowHelp.match_any(&key){
  // Show documents
}
```

#### How can user customize their keybinds

The KeyBind macro can also provide a key config example file for users, so they can easily customize
their keybinds.

- `KeyBindEvent::toml_example()` will return the content of the example.
- `KeyBindEvent::to_toml_example(path)` will write the example into a file.

In general, you can have some subcommand to help users generate the config file to their disk. Note:
You can follow other recipes to handle config folder and path; the current example stores the file
in the current folder.

```rust
KeyBindEvent::to_toml_example("keybind.toml")
```

**The Config Content Example**:

```toml
# Close the application
quit = ["Control+c", "Q", "q"]

# Toggle to open/close a widget show all the commands
toggle_help_widget = ["F1", "?"]
```

As you can see, the documentation of the enum will also be included in the config files, so you
don't need to write the same thing twice. Users can customize the keybind as they need; you just
pass the path of the config file when initializing keybindings.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    KeyEvent::init_and_load("keybind.toml")?;
}
```

If the user only customized part of the key config, the system will patch the user's customized
settings onto the default ones. You can learn this in detail with the following use case.

**The Content of User's Config**:

```toml
quit = ["Control+q"]
```

The config can be loaded successfully. After loading, only `Control+q` can quit the application, and
the default keys `Control+c`, `Q`, `q` will not work anymore. The keybinds to open a widget will
remain the same as the default, because the user did not customize them, so the user can still use
`F1` or `?` to open the widget. You also get the benefit of backward compatibility for key configs,
if you only make additions to the key binding enum.

#### How can user know current key

When the TUI application allows customized keybindings, it's helpful to hint to users what the
current key binding is. You can use `fn key_bindings_display()` for this purpose.

```
println!(
    "type {} for help",
    KeyEvent::ShowHelp.key_bindings_display()
);
```

### Summary

With this approach, the following features are supported:

- **User Customization:** Let users adapt the app to their muscle memory and workflows.
- **Maintainability:** Adding new actions or keys shouldn’t break old configs.
- **Upgradeability:** Users can partially override configs, even as your keybindings evolve.
- **Multiple Shortcuts:** Map several key combos to a single action.
- **Backward Compatibility:** It can always be compatible with legacy configs, if we only make
  additions to the Enum.
- **Better User Experience:** Power users and international users can adjust keyboard layouts as
  needed.

There are some constraints with this approach you need to know ahead of time:

- Always use the enum for new Key Bindings; do not directly handle keycode in functions
- Only make additions to the enum to keep keybind config backward compatibility.
- Using macros will slightly increase compiling time, but this is not easy to detect with modern
  computers.

This approach is similar to [gitui](https://github.com/gitui-org/gitui), though gitui uses RON
format while this example uses TOML format. The TOML example feature is similar to
[kld](https://github.com/kuutamolabs/kld). The KeyBind derive macro combines these patterns into a
complete solution. It's also possible to use `crossterm-keybind-core` alone to achieve a similar
approach with a different pattern.

## Migration guide for existing applications

You do not need to worry that the application will break if some keybinds are not migrated into the
enum. The following guide helps you complete the migration without issues.

- Create a keybind enum first, and initialize it at the start of main
  - You can use different naming for the enum to avoid confusion, for example `AppEvent`, not
    `KeyEvent`.
  - Use `AppEvent::init_and_load(None)?` first
- Gradually move crossterm::KeyEvent into the `match_any` of the enum
  - Normally the condition will change from `match` arms to `if` arms in this step
  - A simple search for `KeyCode`, `KeyModifiers` is good enough rather than searching for
    `KeyEvent`
- Make sure `crossterm::KeyCode` or `crossterm::KeyModifiers` are not being used in your project
  - If `KeyCode` and `KeyModifiers` are not directly used, and are managed by the KeyBind enum
- Allow users to customize the keybind
  - Save the key config to disk with `AppEvent::to_toml_example("keybind.toml")`
  - Then use `AppEvent::init_and_load("keybind.toml")?` to load the customized config

## Extras: A starter template

If you want a ready-made starting point that applies these ideas, here's a template that puts it all
together.

### Option 1. Using GitHub Template

Click the top-left green `Use this template` button of
[ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template).

Set up your project name.

### Option 2. Clone from a GitHub Template

Begin your project from
[ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template):

```bash
git clone https://github.com/yanganto/ratatui-keybind-template.git
cd ratatui-keybind-template
cargo run
```

## References

- [ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template)
- [crossterm-keybind crate](https://github.com/yanganto/crossterm-keybind)
- [Pull request discussion/background](https://github.com/ratatui/templates/pull/124)

With this approach, you can let contributors and users maintain their own keyboard preferences,
reducing maintenance burden and increasing adoption of your Ratatui-based apps.
