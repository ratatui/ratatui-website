---
title: Config.rs
sidebar:
  order: 7
---

At the moment, our keys are hard coded into the app.

```rust {filename="components/home.rs"}
impl Component for Home {

  fn handle_key_events(&mut self, key: KeyEvent) -> Action {
    match self.mode {
      Mode::Normal | Mode::Processing => {
        match key.code {
          KeyCode::Char('q') => Action::Quit,
          KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
          KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
          KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Suspend,
          KeyCode::Char('?') => Action::ToggleShowHelp,
          KeyCode::Char('j') => Action::ScheduleIncrement,
          KeyCode::Char('k') => Action::ScheduleDecrement,
          KeyCode::Char('/') => Action::EnterInsert,
          _ => Action::Tick,
        }
      },
      Mode::Insert => {
        match key.code {
          KeyCode::Esc => Action::EnterNormal,
          KeyCode::Enter => Action::EnterNormal,
          _ => {
            self.input.handle_event(&crossterm::event::Event::Key(key));
            Action::Update
          },
        }
      },
    }
  }
```

If a user wants to press `Up` and `Down` arrow key to `ScheduleIncrement` and `ScheduleDecrement`,
the only way for them to do it is having to make changes to the source code and recompile the app.
It would be better to provide a way for users to set up a configuration file that maps key presses
to actions.

For example, assume we want a user to be able to set up a keyevents-to-actions mapping in a
`config.toml` file like below:

```toml
[keymap]
"q" = "Quit"
"j" = "ScheduleIncrement"
"k" = "ScheduleDecrement"
"l" = "ToggleShowHelp"
"/" = "EnterInsert"
"ESC" = "EnterNormal"
"Enter" = "EnterNormal"
"Ctrl-d" = "Quit"
"Ctrl-c" = "Quit"
"Ctrl-z" = "Suspend"
```

We can set up a `Config` struct using
[the excellent `config` crate](https://docs.rs/config/0.13.3/config/):

```rust
use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::eyre::Result;
use ratatui::crossterm::event::KeyEvent;
use serde::Deserialize;

use crate::action::Action;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub data_dir: PathBuf,
    #[serde(default)]
    pub config_dir: PathBuf,
}

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct KeyBindings(pub HashMap<Mode, HashMap<Vec<KeyEvent>, Action>>);

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default, flatten)]
    pub config: AppConfig,
    #[serde(default)]
    pub keybindings: KeyBindings,
    #[serde(default)]
    pub styles: Styles,
}
```

## Key Bindings and Styles

We are using `serde` to deserialize from a TOML file.

Now the default `KeyEvent` serialized format is not very user friendly, so let's implement our own
version:

```rust
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct KeyBindings(pub HashMap<Mode, HashMap<Vec<KeyEvent>, Action>>);

impl<'de> Deserialize<'de> for KeyBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parsed_map = HashMap::<Mode, HashMap<String, Action>>::deserialize(deserializer)?;

        let keybindings = parsed_map
            .into_iter()
            .map(|(mode, inner_map)| {
                let converted_inner_map = inner_map
                    .into_iter()
                    .map(|(key_str, cmd)| (parse_key_sequence(&key_str).unwrap(), cmd))
                    .collect();
                (mode, converted_inner_map)
            })
            .collect();

        Ok(KeyBindings(keybindings))
    }
}
```

Now all we need to do is implement a `parse_key_event` function.
[You can check the source code for an example of this implementation](https://github.com/ratatui/templates/blob/main/component/template/src/config.rs#L150-L154).

:::tip

You can create different keyevent presses to map to different actions based on the mode of the app
by adding more sections into the toml configuration file.

:::

And in the `handle_key_events` we get the `Action` that should to be performed from the `HashMap`
directly.

```rust
impl App {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }
}
```

In the template, it is set up to handle `Vec<KeyEvent>` mapped to an `Action`. This allows you to
map for example:

- `<g><j>` to `Action::GotoBottom`
- `<g><k>` to `Action::GotoTop`

Here's the JSON configuration we use for the template:

```json
{{#include @code/templates/async-template-counter/.config/config.json5}}
```

Similarly, we have a `Styles` struct that parses custom styles from a config file.

```rust
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct Styles(pub HashMap<Mode, HashMap<String, Style>>);

impl<'de> Deserialize<'de> for Styles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parsed_map = HashMap::<Mode, HashMap<String, String>>::deserialize(deserializer)?;

        let styles = parsed_map
            .into_iter()
            .map(|(mode, inner_map)| {
                let converted_inner_map = inner_map
                    .into_iter()
                    .map(|(str, style)| (str, parse_style(&style)))
                    .collect();
                (mode, converted_inner_map)
            })
            .collect();

        Ok(Styles(styles))
    }
}
```

There are some helper functions in the `config.rs` file that you can use to parse the styles and
keybinds.

## XDG Directories

The template has two main directories that are used for storing configuration files and data files.

Using the directories crate, we can get the XDG directories for the current user. This allows us to
store the configuration and data files in a platform-agnostic way.

```rust
lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref CONFIG_FOLDER: Option<PathBuf> =
        env::var(format!("{}_CONFIG", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
}

// -- snip --
pub fn get_data_dir() -> PathBuf {
    let directory = if let Some(s) = DATA_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".data")
    };
    directory
}

pub fn get_config_dir() -> PathBuf {
    let directory = if let Some(s) = CONFIG_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".config")
    };
    directory
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "kdheepak", env!("CARGO_PKG_NAME")) // Replace kdheepak with your name/project name.
}

```

## Final Code

```rust
{{#include @code/templates/components_async/src/config.rs:all}}
```
