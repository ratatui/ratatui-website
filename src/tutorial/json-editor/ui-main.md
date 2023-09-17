# The Main screen

Because we want the `Main` screen to be rendered behind the editing popup, we will draw it first,
and then have additional logic about our popups

## Our layout

Now that we have our `Frame`, we can actually begin drawing widgets onto it. We will begin by
creating out layout.

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/ui.rs:ui_layout}}
```

The variable `chunks` now contains a length 3 array of `Rect` objects that contain the top left
corner of their space, and their size. We will use these later, after we prepare our widgets.

## The title

The title is an important piece for any application. It helps the user understand what they can do
and where they are. To create our title, we are going to use a `Paragraph` widget (which is used to
display only text), and we are going to tell that `Paragraph` we want a border all around it by
giving it a `Block` with borders enabled. (See [How-To: Block](../../how-to/widgets/block.md) and
[How-To: Paragraph](../../how-to/widgets/paragraph.md) for more information about `Block` and
`Paragraph`).

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/ui.rs:title_paragraph}}
```

In this code, the first thing we do, is create a `Block` with all borders enabled, and the default
style. Next, we created a paragraph widget with the text "Create New Json" styled green. (See
[How-To: Paragraphs](../../how-to/widgets/paragraph.md) for more information about creating
paragraphs and [How-To: Styling-Text](../../how-to/render/style-text.md) for styling text) Finally,
we call `render_widget` on our `Frame`, and give it the widget we want to render it, and the `Rect`
representing where it needs to go and what size it should be. (this is the way all widgets are
drawn)

## The list of existing pairs

We would also like the user to be able to see any key-value pairs that they have already entered.
For this, we will be using another widget, the `List`. The list is what it sounds like - it creates
a new line of text for each `ListItem`, and it supports passing in a state so you can implement
selecting items on the list with little extra work. We will not be implementing selection, as we
simply want the user to be able to see what they have already entered.

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/ui.rs:key_value_list}}
```

For more information on Line, Span, and Style see
[How-To: Displaying Text](../../how-to/render/display-text.md)

In this piece of the function, we create a vector of `ListItem`s, and populate it with styled and
formatted key-value pairs. Finally, we create the `List` widget, and render it.

## The bottom navigational bar

It can help new users of your application, to see hints about what keys they can press. For this, we
are going to implement two bars, and another layout. These two bars will contain information on 1)
The current screen (`Main`, `Editing`, and `Exiting`), and 2) what keybinds are available.

Here, we will create a `Vec` of `Span` which will be converted later into a single line by the
`Paragraph`. (A `Span` is different from a `Line`, because a `Span` indicates a section of `Text`
with a style applied, and doesn't end with a newline)

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/ui.rs:lower_navigation_current_screen}}
```

Next, we are also going to make a hint in the navigation bar with available keys. This one does not
have several sections of text with different styles, and is thus less code.

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/ui.rs:lower_navigation_key_hint}}
```

Finally, we are going to create our first nested layout. Because the `Layout.split` function
requires a `Rect`, and not a `Frame`, we can pass one of our chunks from the previous layout as the
space for the new layout. If you remember the bottom most section from the above graphic:

```svgbob
+----------------------------------+ Constraint::Length  == 3
|       This section should        |
|     always be 3 lines tall       |
|                                  |
+----------------------------------+
```

We will create a new layout in this space by passing it (`chunks[2]`) as the parameter for `split`.

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/ui.rs:lower_navigation_layout}}
```

This code is the visual equivalent of this:

```svgbob
+---------------------------------+ Constraint::Length  == 3
|                |                |
| Length == 50%  | Length == 50%  |
|                |                |
+---------------------------------+
```

And now we can render our footer paragraphs in the appropriate spaces.

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/ui.rs:lower_navigation_rendering}}
```
