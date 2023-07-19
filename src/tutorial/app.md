# App.rs

An often used model for smaller `ratatui` applications, is to have one, large application state struct called `App` or some variant of that name. We will be using this paradigm in this application.
(Read [Concepts]() for other models. [TODO])

## Application modes
Often it is beneficial to imagine that there are seval 'modes' the application can be in. This thinking carries true all the way from input pattern (e.g. Vim vs Nano keybindings, vim's many modes) to supporting several windows in one application (e.g. summary, edit, history, etc) to display different information and preset the user the many options your application has. 

In this tutorial application, we will have three "screens":
- "Normal": the main summary screen showing all past key-value pairs entered
- "Editing": the screen shown when the user wishes to create a new key-value pair
- "Exiting": displays a prompt asking if the user is sure they wish to exit, and asks if they want to output the key-value pairs they have entered.

We represent these possible modes with a simple enum:

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/app.rs:screen_modes}}
```

## Other state enums
Because `ratatui` redraws the entire screen every frame (See [TODO] for more), the programmer is responsible for handling all state. In this case, we will allow the user to input two strings in the `Editing` mode - a key and a value. The programmer is responsible for knowing which the user is trying to edit. 

For this purpose, we will create another enum for our application state called `CurrentlyEditing` to keep track of what the user is currently entering:

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/app.rs:currently_editing}}
```

We also need to keep track of the key-value pairs the user has already entered. To do this, we will create a serde serializable struct, and store a `Vec` of them in our application state struct.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/app.rs:imports}}
```
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/app.rs:key_value_pair}}
```



## The full application state
Now that we have enums to help us track where the user is, it is time to create the struct that actually stores this data, and will be passed around where it is needed.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/app.rs:app_fields}}
```

