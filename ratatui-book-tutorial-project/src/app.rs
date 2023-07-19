// ANCHOR: all
//
// ANCHOR: imports
use serde::{Deserialize, Serialize};
use serde_json::Result;
// ANCHOR_END: imports

// ANCHOR: screen_modes
pub enum CurrentScreen {
    Normal,
    Editing,
    Exiting,
}
// ANCHOR_END: screen_modes

// ANCHOR: currently_editing
pub enum CurrentlyEditing {
    Key,
    Value,
}
// ANCHOR_END: currently_editing

// ANCHOR: app_fields
pub struct App {
    pub key_input: String, // the currently being edited json key.
    pub value_input: String, // the currently being edited json value.
    pub pairs: Vec<KeyValuePair>, // an expanding vector that contains the user's already entered key-value pairs.
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}
// ANCHOR_END: app_fields

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: Vec::new(),
            current_screen: CurrentScreen::Normal,
            currently_editing: None,
        }
    }

    pub fn add_key_value(&mut self) {
        self.pairs.push(KeyValuePair {
            key: self.key_input.clone(),
            value: self.value_input.clone(),
        });
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
        self.current_screen = CurrentScreen::Normal;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}

// ANCHOR: key_value_pair
#[derive(Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}
// ANCHOR_END: key_value_pair

// ANCHOR_END: all
