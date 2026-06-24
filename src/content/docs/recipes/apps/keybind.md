---
title: Configurable Keybindings
sidebar:
  order: 11
  label: Configurable Vim Keybind Config
---

# Recipe: Configurable Keybindings in Ratatui Apps

This recipe explores how to add customizable, user-driven keybindings to your Ratatui application.
It covers common approaches for managing keybindings, supporting user configuration, and maintaining
backward compatibility as your application evolves. We provide a design pattern here, concrete
implementations using the [`crossterm-keybind`](https://github.com/yanganto/crossterm-keybind) or
[`keybind-rs`](https://github.com/rhysd/keybinds-rs) are presented as examples, and also a general
migration guide for an existing tui project.

## Problem & motivation

With growing userbases, developers of Terminal UI (TUI) apps often get requests for alternative
keybinding schemes (like vim-style bindings or personalized shortcuts). Manually supporting such
requests quickly becomes a maintenance burden, and as your app evolves, users expect their custom
keybinds to remain compatible across updates.

## Core idea: separating input from intent

There may be more possible ways to solve this problem, and most problems can be solved by an
intermediate abstraction layer. Configurable keybindings are one such problem that benefits from
this approach. The abstraction layer could be a module, a struct/enum with a set of functions, or a
combination of these.

Within this abstraction, other functions/handlers are not directly comparing the raw key events,
which are the user's key strikes. They just pass the raw event to the abstraction layer, and then
the layer, based on the user's key strikes, executes the corresponding functions or returns back an
_event token_ to let other functions know how to handle it.

As you can imagine, one of the functions in the abstraction layer will read the user's key strikes,
and read a config file on disk like the following, then find out the user's meaning.

However, user inputs are fragile and hard to trust, and frequently checking the config file on disk
is not efficient. So we normally need another function in the abstraction layer to read the file
from disk and deserialize it into memory. This way we can normalize and report possible malformed
user input at first, while the previous function performs comparison in memory in an efficient way.

## Minimal abstraction (no crates)

First, a user provide a file input to the program know keybinds are using for different intention.

_keybind.txt_ - this file is possible in anykind of format or ordering, here is just an example

```text
...
Control+c -> Close the app
...
```

Following is a simple pseudo code to use a simple str for the _event token_, and read from the
keybind.txt provided by user.

```rust
// keybind.rs
use std::fs::read_to_string;

fn known_from_user_strikes(key: crossterm::KeyEvent) -> String {
  for line in read_to_string("keybind.txt").unwrap().lines() {
    if let Some((user_keybind_input, user_intention)) = line.split_once(" -> ") {
      // comparing the keyevent with user input by pseudo function keybind_match
      if keybind_match(key, user_keybind_input) {
        return user_intention.to_string()
      }
    }
  }
  "nothing to do".to_string()
}
```

With a file-based input, users can easily use different key bindings for different actions. As we
mentioned befored, preloading the config file can report error earlier and in a better efficient
way. Following are a simple pseudo code for loading the config, and we can a simple str for the
_event token_.

```rust
// keybind.rs
use std::fs::read_to_string;

fn load_from_user_config() -> Result<(), ()> {
  for line in read_to_string("keybind.txt").unwrap().lines() {
    let Some((user_keybind_input, user_intention)) = line.split_once(" -> ") else {
      // Error handle
    }
    // Normalize users' input and save into memory, and also handle more possible errors
  }
  Ok(())
}
```

With good handling of user input parsing, error handling, and event comparison in these two
functions (one for config parsing and one for event matching), you can complete a configurable
keybindings feature for a TUI app in a 0-dependency way. However, keybinding issues involve more
than just these concerns, so we encourage you to read more and develop the best solution for your
needs.

## Design choices and constraints

The following examples use an approach that defines all keybindings in _a single enum_, in which the
_event tokens_ from the previous section are the enum variants. We are not saying that using an enum
is always the best practice, and some
[discussion](https://github.com/ratatui/ratatui/discussions/627) about using a struct as the _event
token_. Here are some concrete implementations with more detail solutions for you to solve
keybind-related problems ahead.

## Concrete implementations (crates as examples)

### Crossterm-keybind

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

And following methods are implemented.

- Initialize and read user's config
  - `KeyEvent::init_and_load(Some(PathBuf::from("/The/path/to/keyconfig.toml")))?`
- Know the user's intention
  - `KeyBindEvent::Quit.match_any(&key)`
  - `for event in KeyBindEvent::dispatch(&key) {...}`
- Provide default configure with documentation
  - `KeyBindEvent::toml_example()` will return the content of the example.
  - `KeyBindEvent::to_toml_example(path)` will write the example into a file.
- Provide hint for current keybind
  - `Quit.key_bindings_display()` to print current keybind in chars.

### Keybind-rs

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

And following methods are implemented. `if let Some(action) = keybinds.dispatch(&event) {...}`

### Summary

With these approaches, the benefits of configurable keybinding and additional features provided by
third party crates are listed in the following, making it easier for you to find your solution:

- **User Customization:** Let users adapt the app to their muscle memory and workflows.
- **Multiple Shortcuts:** Map several key combos to a single action.
- **Better User Experience:** Power users and international users can adjust keyboard layouts.
- **Backward Compatibility(crossterm-keybind):** It can always be compatible with legacy configs, if
  we only make additions to the Enum.
- **Maintainability(crossterm-keybind):** It is easy to keep a keybind config with document updated
  with the code.
- **Better Developer Experience(crossterm-keybind):** Easy to setup default keybindings.
- **Flexible Keybindings(crossterm-keybind):** It is possible to trigger multiple enum variants from
  one keybinding.
- **Keybind Hint(crossterm-keybind):** easier to know what the current keybind is.
- **Embedded Config(keybind-rs):** Keyboard can be part of the main config.
- **Customizable Deserialization(keybind-rs):** Customizable deserializer for the config.
- **Emacs-style(keybind-rs):** Using Emacs style keybinds and map multiple key strikes to an event.

There are some constraints with these approaches you need to know ahead of time:

- Always use the enum for new key bindings; do not directly handle keycode in functions.
- Using macros will slightly increase compile time, but this is not easy to detect with modern
  computers.
- Only make additions to the enum to keep keybind config backward compatibility (crossterm-keybind).
- One keybind can only trigger one enum variant (keybind-rs).

`crossterm-keybind` is a crate opened to used with features with less codding, and it's still
possible to use `crossterm-keybind-core` alone to achieve a different approach. On the other hand,
`keybind-rs` is a lightweight intended crate.

## Migration guide

You do not need to worry that the application will break if some keybinds are not migrated into the
enum. The following guide helps you complete the migration without issues.

- Create a keybind enum first, and initialize it at the start of main
  - You can use different naming for the enum to avoid confusion, for example `AppEvent`, not
    `KeyEvent`.
  - (manual) Add `fn load_from_user_config()`
  - (crossterm-keybind) Use `AppEvent::init_and_load(None)?`
  - (keybind-rs) Add deserializer for your config
- Gradually move crossterm::KeyEvent into the `match_any` (crossterm-keybind) or `dispatch` of the
  enum, or manually create a `fn known_from_user_strikes(key: crossterm::KeyEvent)`
  - Normally the condition will change from `match` arms to `if` arms in this step
  - A simple search for `KeyCode`, `KeyModifiers` is good enough rather than searching for
    `KeyEvent`
- Make sure `crossterm::KeyCode` or `crossterm::KeyModifiers` are not being used directly in your
  project
- Verify that `KeyCode` and `KeyModifiers` are managed through the KeyBind enum
- Allow users to customize the keybind
  - (manual, keybind-rs) Manually provide example for keybind
  - (crossterm-keybind) Save the key config to disk with
    `AppEvent::to_toml_example("keybind.toml")`, and use `AppEvent::init_and_load("keybind.toml")?`
    to load the customized config
- (Optional) Provide keybind hint in UI
  - (crossterm-keybind) Using `Quit.key_bindings_display()` to print current keybind in chars in ui.

## Optional templates & references

If you want a ready-made starting point that applies these ideas, here's a template that puts it all
together.

### Option 1. Using GitHub Template for crossterm-keybind

Click the top-left green `Use this template` button of
[ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template).

or clone it `git clone https://github.com/yanganto/ratatui-keybind-template.git`

### Option 2. Check examples from keybind-rs

Follow [examples](https://github.com/rhysd/keybinds-rs/blob/main/examples) from keybind-rs.

### References

- [ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template)
- [crossterm-keybind crate](https://github.com/yanganto/crossterm-keybind)
- [keybind-rs crate](https://github.com/rhysd/keybinds-rs)
- [Pull request discussion/background](https://github.com/ratatui/templates/pull/124)

With this approach, you can let contributors and users maintain their own keyboard preferences,
reducing maintenance burden and increasing adoption of your Ratatui-based apps.
