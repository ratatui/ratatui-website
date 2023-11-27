---
title: App.rs
---

As we saw in the previous section, a common model for smaller `ratatui` applications is to have one
application state struct called `App` or some variant of that name. We will be using this paradigm
in this application as well.

This struct will contain all of our "persistent" data and will be passed to any function that needs
to know the current state of the application.

<!--
    (Read [Application Pattern Concepts](../concepts/storing_state) to explore some other models)
-->

## Application modes

It is useful to think about the several "modes" that your application can be in. Thinking in "modes"
will make it easier to segregate everything from what window is getting drawn, to what keybinds to
listen for.

We will be using the application's state to track two things:

1. what screen the user is seeing,
2. which box should be highlighted, the "key" or "value" (this only applies when the user is editing
   a key-value pair).

### Current Screen Enum

In this tutorial application, we will have three "screens":

- `Main`: the main summary screen showing all past key-value pairs entered
- `Editing`: the screen shown when the user wishes to create a new key-value pair
- `Exiting`: displays a prompt asking if the user wants to output the key-value pairs they have
  entered.

We represent these possible modes with a simple enum:

```rust
{{#include @code/ratatui-json-editor-app/src/app.rs:screen_modes}}
```

### Currently Editing Enum

As you may already know, `ratatui` does not automatically redraw the screen[^note]. `ratatui` also
does not remember anything about what it drew last frame.

This means that the programmer is responsible for handling all state and updating widgets to reflect
changes. In this case, we will allow the user to input two strings in the `Editing` mode - a key and
a value. The programmer is responsible for knowing which the user is trying to edit.

For this purpose, we will create another enum for our application state called `CurrentlyEditing` to
keep track of which field the user is currently entering:

```rust
{{#include @code/ratatui-json-editor-app/src/app.rs:currently_editing}}
```

## The full application state

Now that we have enums to help us track where the user is, we will create the struct that actually
stores this data which can be passed around where it is needed.

```rust
{{#include @code/ratatui-json-editor-app/src/app.rs:app_fields}}
```

## Helper functions

While we could simply keep our application state as simply a holder of values, we can also create a
few helper functions which will make our life easier elsewhere. Of course, these functions should
only affect the application state itself, and nothing outside of it.

### `new()`

We will be adding this function simply to make creating the state easier. While this could be
avoided by specifying it all in the instantiation of the variable, doing it here allows for easy to
change universal defaults for the state.

```rust
{{#include @code/ratatui-json-editor-app/src/app.rs:impl_new}}
    // --snip--
```

### `save_key_value()`

This function will be called when the user saves a key-value pair in the editor. It adds the two
stored variables to the key-value pairs `HashMap`, and resets the status of all of the editing
variables.

```rust
    // --snip--
{{#include @code/ratatui-json-editor-app/src/app.rs:save_key_value}}
    // --snip--
```

### `toggle_editing()`

Sometimes it is easier to put simple logic into a convenience function so we don't have to worry
about it in the main code block. `toggle_editing` is one of those cases. All we are doing, is
checking if something is currently being edited, and if it is, swapping between editing the Key and
Value fields.

```rust
    // --snip--
{{#include @code/ratatui-json-editor-app/src/app.rs:toggle_editing}}
    // --snip--
```

### `print_json()`

Finally, is another convenience function to print out the serialized json from all of our key-value
pairs.

```rust
    // --snip--
{{#include @code/ratatui-json-editor-app/src/app.rs:print_json}}
    // --snip--
```

<!-- prettier-ignore -->
[^note]: In ratatui, every frame draws the UI anew. See the [Rendering section](../../../concepts/rendering) for more information.
