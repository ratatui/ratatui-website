---
title: Configurable Keybindings
sidebar:
  order: 11
  label: Configurable Vim Keybind Config
---

# Recipe: Configurable Keybindings in Ratatui Apps

This recipe explores how to add customizable, user-driven keybindings to your Ratatui application.
It covers common approaches for managing keybindings, supporting user configuration, and maintaining
backward compatibility as your application evolves. Concrete implementations using the
[`crossterm-keybind`](https://github.com/yanganto/crossterm-keybind) or
[`keybind-rs`](https://github.com/rhysd/keybinds-rs) are presented as examples.

## Problem statement and motivation

With growing userbases, developers of Terminal UI (TUI) apps often get requests for alternative
keybinding schemes (like vim-style bindings or personalized shortcuts). Manually supporting such
requests quickly becomes a maintenance burden, and as your app evolves, users expect their custom
keybinds to remain compatible across updates.

## Design and Constraints

### Core Pattern

The main idea is to define all keybindings in _a single enum_.

_The Enum Example from crossterm-keybind:_

```rust
use crossterm_keybind::KeyBind;

#[derive(KeyBind)]
pub enum KeyEvent {
    /// Close the application
    #[keybindings["Control+c", "Q", "q"]]
    Quit,

    /// Toggle to open/close a widget showing all the commands
    #[keybindings["h", "F1"]]
    ShowHelp,
}
```

_The Enum Example from keybinds-rs:_

```rust
#[derive(Deserialize)]
enum Action {
    Exit,
    End,
}

#[derive(Deserialize)]
struct Config {
    keyboard: Keybinds<Action>,
}
```

The difference is that `crossterm-keybind` uses attribute macros to declare default shortcuts and
supports external TOML configuration for overrides and patches.

On the other hand, `keybinds-rs` leaves default keybinding definitions flexible, allowing developers
to provide them through any deserialization approach they prefer.

#### How to capture user input

Within an abstraction, the enum, you don't want to directly compare the `KeyCode`, `KeyModifiers` of
a `crossterm::KeyEvent` when capturing a user's input. Instead, you can pass a reference of it to a
`match_any` method with `crossterm-keybind` or `dispatch` from `keybind-rs`, which can be provided
by the derive macro. Before you match any keyevent, you should initialize a keybind instance for
your app first, because it is possible for your users to have customized keybinds (this will be
explained in the next section).

- `crossterm-keybind` uses `KeyEvent::init_and_load(...)`
- `keybind-rs` needs developers to construct the instance via a deserializer

Normally, you can run initialization as the first task of the main function.

_The Example from crossterm-keybind:_

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

or

```rust
for event in KeyBindEvent::dispatch(&key) {
  match event {
    KeyBindEvent::Quit => {
      // Close the app
    },
    KeyBindEvent::ShowHelp => {
      // Show documents
    },
  }
}
```

_The Example from keybinds-rs:_

```rust
    // Parse the configuration from the file content
    let config: Config = toml::from_str(CONFIG_FILE_CONTENT).unwrap();

    // `Keybinds` instance is a key bindings dispatcher that receives key inputs and
    // dispatches the corresponding actions.
    let mut keybinds = config.keyboard;
```

```rust
  if let Some(action) = keybinds.dispatch(&event) {
      match action {
          Action::Exit => break,
          Action::End => execute!(stdout, cursor::MoveRight(9999))?,
      }
  }
```

#### How can users customize their keybinds

The ways to customize keybindings differ between the two crates.

**The crossterm-keybind way**

Crossterm additionally takes care of:

- Override issues using the struct-patch feature, which is borrowed from
  [gitui](https://github.com/gitui-org/gitui)
- Config file documentation, which is adapted from [kld](https://github.com/kuutamolabs/kld)

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

_The crossterm-keybind Config Content Example_

```toml
# Close the application
quit = ["Control+c", "Q", "q"]

# Toggle to open/close a widget showing all the commands
show_help = ["h", "F1"]
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

_The Content of User's Config:_

```toml
quit = ["Control+q"]
```

The config can be loaded successfully. After loading, only `Control+q` can quit the application, and
the default keys `Control+c`, `Q`, `q` will not work anymore. The keybinds to open a widget will
remain the same as the default, because the user did not customize them, so the user can still use
`F1` or `?` to open the widget. You also get the benefit of backward compatibility for key configs,
if you only make additions to the key binding enum.

It is possible to let a keybind trigger more than one enum variant.

**The keybind-rs way**

Developers can provide multiple sets of keybindings in the config file. The keybinds to the enum can
be many-to-one (multiple keybinds mapping to the same action). The keyboard configuration can be
part of your main config file.

_The Example Content of User's Config:_

```toml
const CONFIG_FILE_CONTENT: &str = r#"
[keyboard]
# Standard bindings
"End" = "End"
"Mod+q" = "Exit"

# Emacs-like bindings
"Ctrl+e" = "End"
"Ctrl+x Ctrl+c" = "Exit"

# Vim-like bindings
"$" = "End"
"Esc" = "Exit"
"#;
```

Every keybind binds to one enum variant.

#### How can users know the current keybindings

When the TUI application allows customized keybindings, it's helpful to show users what the current
keybindings are. `crossterm-keybind` provides a `key_bindings_display()` method for this purpose.

```rust
println!(
    "type {} for help",
    KeyEvent::ShowHelp.key_bindings_display()
);
```

### Summary

With these approaches, the following features are supported:

Both crates support:

- **User Customization:** Let users adapt the app to their muscle memory and workflows.
- **Multiple Shortcuts:** Map several key combos to a single action.
- **Better User Experience:** Power users and international users can adjust keyboard layouts.

crossterm-keybind supports:

- **Backward Compatibility:** It can always be compatible with legacy configs, if we only make
  additions to the Enum.
- **Maintainability:** It is easy to keep a keybind config updated with the code.
- **Better Developer Experience:** Easy to setup default keybindings.
- **Flexible Keybindings:** It is possible to trigger multiple enum variants from one keybinding.

keybind-rs supports:

- **Embedded Config:** Keyboard can be part of the main config.
- **Customizable Deserialization:** Customizable deserializer for the config.

There are some constraints with these approaches you need to know ahead of time:

Both crates have constraints:

- Always use the enum for new key bindings; do not directly handle keycode in functions.
- Using macros will slightly increase compile time, but this is not easy to detect with modern
  computers.

crossterm-keybind constraints:

- Only make additions to the enum to keep keybind config backward compatibility.

keybind-rs constraints:

- One keybind can only trigger one enum variant.

It's also possible to use `crossterm-keybind-core` alone to achieve a similar approach with a
different pattern.

## Migration guide for existing applications

You do not need to worry that the application will break if some keybinds are not migrated into the
enum. The following guide helps you complete the migration without issues.

- (Both) Create a keybind enum first, and initialize it at the start of main
  - You can use different naming for the enum to avoid confusion, for example `AppEvent`, not
    `KeyEvent`.
  - (crossterm-keybind) Use `AppEvent::init_and_load(None)?` first
  - (keybind-rs) Add deserializer for your config
- (Both) Gradually move crossterm::KeyEvent into the `match_any` (crossterm-keybind) or `dispatch`
  (keybind-rs) of the enum
  - Normally the condition will change from `match` arms to `if` arms in this step
  - A simple search for `KeyCode`, `KeyModifiers` is good enough rather than searching for
    `KeyEvent`
- (Both) Make sure `crossterm::KeyCode` or `crossterm::KeyModifiers` are not being used directly in
  your project
  - Verify that `KeyCode` and `KeyModifiers` are managed through the KeyBind enum
- Allow users to customize the keybind
  - (crossterm-keybind) Save the key config to disk with `AppEvent::to_toml_example("keybind.toml")`
  - (crossterm-keybind) Then use `AppEvent::init_and_load("keybind.toml")?` to load the customized
    config
  - (keybind-rs) Manually provide example for keybind

## Extras: starter templates

If you want a ready-made starting point that applies these ideas, here's a template that puts it all
together.

### Option 1. Using GitHub Template for crossterm-keybind

Click the top-left green `Use this template` button of
[ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template).

Set up your project name.

or simply clone the project

```bash
git clone https://github.com/yanganto/ratatui-keybind-template.git
cd ratatui-keybind-template
cargo run
```

### Option 2. Check examples from keybind-rs

Follow [examples](https://github.com/rhysd/keybinds-rs/blob/main/examples) from keybind-rs.

## References

- [ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template)
- [crossterm-keybind crate](https://github.com/yanganto/crossterm-keybind)
- [keybind-rs crate](https://github.com/rhysd/keybinds-rs)
- [Pull request discussion/background](https://github.com/ratatui/templates/pull/124)

With this approach, you can let contributors and users maintain their own keyboard preferences,
reducing maintenance burden and increasing adoption of your Ratatui-based apps.
