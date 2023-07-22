# App.rs

An often used model for smaller `ratatui` applications, is to have one, large application state struct called `App` or some variant of that name. We will be using this paradigm in this application.
(Read [Concepts]() for other models. [TODO])

## Application modes
Often it is beneficial to imagine that there are seval 'modes' the application can be in. This thinking carries true all the way from input pattern (e.g. Vim vs Nano keybindings, vim's many modes) to supporting several windows in one application (e.g. summary, edit, history, etc) to display different information and preset the user the many options your application has. 

In this tutorial application, we will have three "screens":
- "Main": the main summary screen showing all past key-value pairs entered
- "Editing": the screen shown when the user wishes to create a new key-value pair
- "Exi
ting": displays a prompt asking if the user is sure they wish to exit, and asks if they want to output the key-value pairs they have entered.

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
Now that we have enums to help us track where the user is, we will create the struct that actually stores this data which can be passed around where it is needed.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/app.rs:app_fields}}
```

## Helper functions
While we could simply keep our application state as only a holder of values, we can also create a few helper functions which will make our life easier elsewhere. Of course, these functions should only affect the application state itself, and nothing outside of it.

#### new()
We will be adding this function simply to make creating the state easier. While this could be avoided by specifying it all in the instantiation of the variable, doing it here allows for easy to change, universal default states.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/app.rs:impl_new}}
    ...
```

#### save_key\_value()
This function will be called when the user saves a key-value pair in the editor. It adds the two stored variables to the key-value pairs vector, and resets the status of all of the editing variables.

```rust,no_run,noplayground
    ...
{{#include ../../ratatui-book-tutorial-project/src/app.rs:save_key_value}}
    ...
```

#### toggle_editing()
Sometimes it is easier to put simple logic into a convenience function so we don't have to worry about it in the main code block. `toggle_editing` is one of those cases. 
All we are doing, is checking if something is currently being edited, and if it is, swapping between editing the Key and Value fields.

```rust,no_run,noplayground
    ...
{{#include ../../ratatui-book-tutorial-project/src/app.rs:toggle_editing}}
    ...
```

#### print_json()
Finally, is another convenience function to print out the serialized json from all of our key-value pairs.

```rust,no_run,noplayground
    ...
{{#include ../../ratatui-book-tutorial-project/src/app.rs:print_json}}
    ...
```


## The finished file
The finished `app.rs` file should look something like this:
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/app.rs:all}}
```
