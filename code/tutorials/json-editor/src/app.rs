// ANCHOR: all
use std::collections::HashMap;

// ANCHOR: screen_modes
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}
// ANCHOR_END: screen_modes

// ANCHOR: edit_focus
#[derive(Clone, Copy)]
pub enum EditFocus {
    Key,
    Value,
}
// ANCHOR_END: edit_focus

// ANCHOR: editing_pair
pub struct EditingPair {
    pub key: String,      // the currently being edited json key.
    pub value: String,    // the currently being edited json value.
    pub focus: EditFocus, // which field the user is editing.
}
// ANCHOR_END: editing_pair

// ANCHOR: app_fields
pub struct App {
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub editing_pair: Option<EditingPair>, // the optional key-value pair currently being edited. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            editing_pair: None,
        }
    }
    // ANCHOR_END: impl_new

    // ANCHOR: start_editing
    pub fn start_editing(&mut self) {
        self.editing_pair = Some(EditingPair {
            key: String::new(),
            value: String::new(),
            focus: EditFocus::Key,
        });
    }
    // ANCHOR_END: start_editing

    // ANCHOR: save_key_value
    pub fn save_key_value(&mut self) {
        if let Some(editing_pair) = self.editing_pair.take() {
            self.pairs.insert(editing_pair.key, editing_pair.value);
        }
    }
    // ANCHOR_END: save_key_value

    // ANCHOR: toggle_editing
    pub fn toggle_editing(&mut self) {
        if let Some(editing_pair) = &mut self.editing_pair {
            match editing_pair.focus {
                EditFocus::Key => editing_pair.focus = EditFocus::Value,
                EditFocus::Value => editing_pair.focus = EditFocus::Key,
            };
        } else {
            self.start_editing();
        }
    }
    // ANCHOR_END: toggle_editing

    // ANCHOR: print_json
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }
    // ANCHOR_END: print_json
}
// ANCHOR_END: all
