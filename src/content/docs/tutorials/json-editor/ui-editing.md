---
title: UI - Editing Popup
sidebar:
  order: 5
---

Now that the `Main` screen is rendered, we now need to check if the `Editing` popup needs to be
rendered. Since the `ratatui` renderer simply writes over the cells within a `Rect` on a
`render_widget`, we simply need to give `render_widget` an area on top of our `Main` screen to
create the appearance of a popup.

## Popup area and title

The first thing we will do, is draw the `Block` that will contain the popup. We will give this
`Block` a title to display as well to explain to the user what it is.

```rust
{{#include @code/ratatui-json-editor-app/src/ui.rs:editing_popup}}
```

## Popup contents

Now that we have where our popup is going to go, we can create the layout for the popup, and create
and draw the widgets inside of it.

First, we will create split the `Rect` given to us by `centered_rect`, and create a layout from it.
Note the use of `margin(1)`, which gives a 1 space margin around any layout block, meaning our new
blocks and widgets don't overwrite anything from the first popup block.

```rust
{{#include @code/ratatui-json-editor-app/src/ui.rs:popup_layout}}
```

Now that we have the layout for where we want to display the keys and values, we will actually
create the blocks and paragraphs to show what the user has already entered.

```rust
{{#include @code/ratatui-json-editor-app/src/ui.rs:key_value_blocks}}
```

Note that we are declaring the blocks as variables, and then adding extra styling to the block the
user is currently editing. Then we create the `Paragraph` widgets, and assign the blocks with those
variables. Also note how we used the `popup_chunks` layout instead of the `popup_block` layout to
render these widgets into.
