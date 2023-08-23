# App.rs

A common model for smaller `ratatui` applications, is to have one large application state struct
called `App` or some variant of that name. We will be using this paradigm in this application.

This struct will contain all of our "persistent" data and will be passed to any function that needs
to know the current state of the application.

(Read [Application Pattern Concepts](../concepts/storing_state.md) to explore some other models)

## Application modes

Often it is beneficial to imagine that there are several 'modes' the application can be in. Thinking
this way will make it easier to segregate everything from what window is getting drawn, to what
keybinds to listen for. We will be using the application's state and enums to track two things. The
first thing we are going to track is what screen the user should be seeing, and the second will be
an optional enum that only applies while the user is editing a key-value pair that tracks which (key
or value) should be highlighted for the user.

In this tutorial application, we will have three "screens":

- `Main`: the main summary screen showing all past key-value pairs entered
- `Editing`: the screen shown when the user wishes to create a new key-value pair
- `Exiting`: displays a prompt asking if the user wants to output the key-value pairs they have
  entered.

We represent these possible modes with a simple enum:

```rust,no_run,noplayground
{{#include ../../../ratatui-book-tutorial-project/src/app.rs:screen_modes}}
```

## Other state enums

`Ratatui` does not automatically redraw the screen (See [Rendering](./../concepts/rendering.md) for
more information), and it does not remember anything about what it drew last frame. This means that
the programmer is responsible for handling all state and updating widgets to reflect changes. In
this case, we will allow the user to input two strings in the `Editing` mode - a key and a value.
The programmer is responsible for knowing which the user is trying to edit.

For this purpose, we will create another enum for our application state called `CurrentlyEditing` to
keep track of which field the user is currently entering:

```rust,no_run,noplayground
{{#include ../../../ratatui-book-tutorial-project/src/app.rs:currently_editing}}
```

## The full application state

Now that we have enums to help us track where the user is, we will create the struct that actually
stores this data which can be passed around where it is needed.

```rust,no_run,noplayground
{{#include ../../../ratatui-book-tutorial-project/src/app.rs:app_fields}}
```

## Helper functions

While we could simply keep our application state as simply a holder of values, we can also create a
few helper functions which will make our life easier elsewhere. Of course, these functions should
only affect the application state itself, and nothing outside of it.

#### new()

We will be adding this function simply to make creating the state easier. While this could be
avoided by specifying it all in the instantiation of the variable, doing it here allows for easy to
change universal defaults for the state.

```rust,no_run,noplayground
{{#include ../../../ratatui-book-tutorial-project/src/app.rs:impl_new}}
    ...
```

#### save_key_value()

This function will be called when the user saves a key-value pair in the editor. It adds the two
stored variables to the key-value pairs `HashMap`, and resets the status of all of the editing
variables.

```rust,no_run,noplayground
    ...
{{#include ../../../ratatui-book-tutorial-project/src/app.rs:save_key_value}}
    ...
```

#### toggle_editing()

Sometimes it is easier to put simple logic into a convenience function so we don't have to worry
about it in the main code block. `toggle_editing` is one of those cases. All we are doing, is
checking if something is currently being edited, and if it is, swapping between editing the Key and
Value fields.

```rust,no_run,noplayground
    ...
{{#include ../../../ratatui-book-tutorial-project/src/app.rs:toggle_editing}}
    ...
```

#### print_json()

Finally, is another convenience function to print out the serialized json from all of our key-value
pairs.

```rust,no_run,noplayground
    ...
{{#include ../../../ratatui-book-tutorial-project/src/app.rs:print_json}}
    ...
```
