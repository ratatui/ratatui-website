---
title: Handle Key Bindings
sidebar:
  order: 11
  label: Key Bindings
---
# Recipe: Configurable Keybindings in Ratatui Apps

With growing userbases, developers of Terminal UI (TUI) apps often get requests for alternative keybinding schemes (like vim-style bindings or personalized shortcuts). Manually supporting such requests quickly becomes a maintenance burden, and as your app evolves, users expect their custom keybinds to remain compatible across updates.

This recipe presents a modular and extensible solution, based on the new [ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template), to add customizable, user-driven keybindings to your Ratatui application. The system uses the [`crossterm-keybind`](https://github.com/yanganto/crossterm-keybind) crate for TOML-based configuration, backward compatibility, and patch-style overrides.

---

## Motivation & Design

- **User customization:** Let users adapt the app to their muscle memory and workflows.
- **Maintainability:** Adding new actions or keys shouldn’t break old configs.
- **Upgradeability:** Users can partially override configs, even as your keybindings evolve.
- **Multiple shortcuts:** Map several key combos to a single action.

The main idea is to define all keybindings in a single enum, use attribute macros to declare default shortcuts, and support external TOML configuration for overrides and patches.

---

## Getting Started

### 1. Start with the Template

Begin your project from [ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template):

```bash
git clone https://github.com/yanganto/ratatui-keybind-template.git
cd ratatui-keybind-template
cargo run
```

### 2. Core File Structure

```
src/
├── main.rs       # Application entry point and terminal setup
├── app.rs        # Application state and logic
├── keybinds.rs   # Keybinding definitions using crossterm-keybind
└── ui.rs         # UI rendering logic
```

---

## Defining Keybindings

All keybindings are defined in a Rust enum in `src/keybinds.rs`, using macros from `crossterm-keybind`.

**Example:**

```rust
use crossterm_keybind::KeyBind;

#[derive(KeyBind)]
pub enum KeyEvent {
    /// Quit the application
    #[keybindings["Control+c", "Q", "q"]]
    Quit,

    /// Show help menu (via 'h' or F1)
    #[keybindings["h", "F1"]]
    ShowHelp,
}
```

---

## Handling Key Events

In your application logic (commonly `src/app.rs`), match events using `.match_any(&key)` for ergonomic, readable code:

```rust
impl App {
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        if KeyBindEvent::Quit.match_any(&key) {
            // Handle quitting
            return false;
        }
        if KeyBindEvent::ShowHelp.match_any(&key) {
            // Show help popup or similar action
            return true;
        }
        true
    }
}
```

---

## Supporting User Configuration

### 1. Generate a Default Keybind Config

You can generate an example TOML file for users to edit:

```rust
use crossterm_keybind::KeyBindTrait;
use crate::keybinds::KeyEvent;

// This creates "keybinds.toml" with the default actions and bindings.
KeyEvent::to_toml_example("keybinds.toml").unwrap();
```

**Example output (`keybinds.toml`):**

```toml
# Quit the application
quit = ["Control+c", "Q", "q"]

# Show help
show_help = ["h", "F1"]
```

### 2. Load Custom Keybindings

To allow users to override with their own `keybinds.toml` at runtime:

```rust
// In main.rs
KeyEvent::init_and_load(Some("keybinds.toml".into()))?;
```

---

## Practical Benefits

- **Multiple keyboard shortcuts** per action, supporting both defaults and user overrides.
- **Backward-compatible:** upgrades to your app or new bindings don't break user configs—they’re patched or merged as needed.
- **Easy extensibility:** Add new actions by updating `KeyEvent` and handling logic.
- **Better user experience:** Power users and international users can adjust keyboard layouts as needed.

---

## References

- [ratatui-keybind-template](https://github.com/yanganto/ratatui-keybind-template)
- [crossterm-keybind crate](https://github.com/yanganto/crossterm-keybind)
- [Pull request discussion/background](https://github.com/ratatui/templates/pull/124)

---

With this approach, you can let contributors and users maintain their own keyboard preferences, reducing maintenance burden and increasing adoption of your Ratatui-based apps.
