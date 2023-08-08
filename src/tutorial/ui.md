# UI.rs

Finally we come to the last piece of the puzzle, and also the hardest part when you are just starting out creating `ratatui` TUIs. The actual UI. If you have created a UI before, you should know that the UI code can take up much more space than you think it should, and this is not exception. 
We will only briefly cover layouts and how this core of `ratatui` design works, but there will be links to more resources where they are covered in depth. 

## Some basics
First, we need to understand how we draw widgets onto the screen in the first place.
The TLDR is, that we create a widget, and pass it to a `Frame` along with a size. How we get that size, is where layouts come in.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:ui_layout}}
```

You can this as an small instruction manual for cutting up a rectangle into smaller rectangles, because that is what is actually happening. 
(Find more information on the different types of constraints, read [How-To: Constraints](./../how-to/layout-constraints.md))
In the example above, you can read the instructions aloud like this: "Take the area f.size (which is a rectangle), and cut it into three vertical pieces (making horizontal cuts). The first section should always be 3 lines tall. The second section should never be smaller than one line tall, but can take extra space if there is any. The final section should also be 3 lines tall".

For those visual learners, I have the following graphic:
```
This outer box is the original Frame,
which we get with frame.size()
------------------------------------ Constraint::Length(3)
|       This section should        |
|     always be 3 lines tall       |
|                                  |
|----------------------------------| Constraint::Min(1)
|      This section should         |
|     never be less than 1         |
|      line tall, but can be       |
|     longer if space is available |
|                                  |
------------------------------------ Constraint::Length(3)
|       This section should        |
|     always be 3 lines tall       |
|                                  |
|----------------------------------|
```

Now that we have that out of the way, let us create the TUI for our application.

## The function signature
Our ui function needs two things to successfully create our UI elements. The `Frame` which contains the size of the terminal at render time (this is important, because it allows us to take resizeable terminals into account), and the application state. 

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:method_sig}}
```

# The Main screen
Because we want the `Main` screen to be rendered behind the editing popup, we will draw it first, and then have additional logic about our popups

## Our layout 
Now that we have our `Frame`, we can actually begin drawing widgets onto it. 
We will begin by creating out layout.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:ui_layout}}
```

The variable `chunks` now contains a length 3 array of `Rect` objects that contain the top left corner of their space, and their size. We will use these later, after we prepare our widgets.

## The title
The title is an important piece for any application. It helps the user understand what they can do and where they are.
To create our title, we are going to use a `Paragraph` widget (which is used to display only text), and we are going to tell that `Paragraph` we want a border all around it by giving it a `Block` with borders enabled. (See [How-To: Block](./../how-to/block.md) and [How-To: Paragraph](./../how-to/paragraph.md) for more information about `Block` and `Paragraph`).

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:title_paragraph}}
```

In this code, the first thing we do, is create a `Block` with all borders enabled, and the default style.
Next, we created a paragraph widget with the text "Create New Json" styled green. (See [How-To: Paragraphs](./../how-to/paragraph.md) for more information about creating paragraphs and [How-To: Styling-Text](./../how-to/styling-text.md) for styling text)
Finally, we call `render_widget` on our `Frame`, and give it the widget we want to render it, and the `Rect` representing where it needs to go and what size it should be. (this is the way all widgets are drawn)

## The list of existing pairs.
We would also like the user to be able to see any key-value pairs that they have already entered.
For this, we will be using another widget, the `List`. The list is what it sounds like - it creates 
a new line of text for each `ListItem`, and it supports passing in a state so you can implement selecting
items on the list with little extra work. We will not be implementing selection, as we simply want the user
to be able to see what they have already entered.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:key_value_list}}
```

For more information on Line, Span, and Style see [How-To: Displaying Text](./../how-to/displaying-text.md)

In this piece of the function, we create a vector of `ListItem`s, and populate it with styled and formatted key-value pairs. Finally, we create the `List` widget, and render it. 

## The bottom navigational bar
It can help new users of your application, to see hints about what keys they can press. For this, we are going to implement two bars, and another layout. 
These two bars will contain information on 1) The current screen (`Main`, `Editing`, and `Exiting`), and 2) what keybinds are available. 

Here, we will create a `Vec` of `Span` which will be converted later into a single line by the `Paragraph`. (A `Span` is different from a `Line`, because a `Span` indicates a section of `Text` with a style applied, and doesn't end with a newline)
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:lower_navigation_current_screen}}
```

Next, we are also going to make a hint in the navigation bar with available keys. This one does not have several sections of text with different styles, and is thus less code.
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:lower_navigation_key_hint}}
```

Finally, we are going to create our first nested layout. Because the `Layout.split` function requires a `Rect`, and not a `Frame`, we can pass one of our chunks from the previous layout as the space for the new layout. 
If you remember the bottom most section from the above graphic:
```
------------------------------------ Constraint::Length(3)
|       This section should        |
|     always be 3 lines tall       |
|                                  |
|----------------------------------|
```

We will create a new layout in this space by passing it (`chunks[2]`) as the parameter for `split`. 

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:lower_navigation_layout}}
```

This code is the visual equivalent of this:
```
----------------------------------- Constraint::Length(3)
|                |                |
| Percentage(50) | Percentage(50) |
|                |                |
|---------------------------------|
```

And now we can render our footer paragraphs in the appropriate spaces.
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:lower_navigation_rendering}}
```

# The Editing Popup
Now that the `Main` screen is rendered, we now need to check if the `Editing` popup needs to be rendered. Since the `ratatui` renderer simply writes over the cells within a `Rect` on a `render_widget`, we simply need to give `render_widget` an area on top of our `Main` screen to create the appearance of a popup.

## Popup area and title
The first thing we will do, is draw the `Block` that will contain the popup. We will give this `Block` a title to display as well to explain to the user what it is. (We will cover `centered_rect` below)
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:editing_popup}}
```

## Popup contents
Now that we have where our popup is going to go, we can create the layout for the popup, and create and draw the widgets inside of it.

First, we will create split the `Rect` given to us by `centered_rect`, and create a layout from it. Note the use of `margin(1)`, which gives a 1 space margin around any layout block, meaning our new blocks and widgets don't overwrite anything from the first popup block. 
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:popup_layout}}
```

Now that we have the layout for where we want to display the keys and values, we will actually create the blocks and paragraphs to show what the user has already entered.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:key_value_blocks}}
```
Note that we are declaring the blocks as variables, and then adding extra styling to the block the user is currently editing. Then we create the `Paragraph` widgets, and assign the blocks with those variables.
Also note how we used the `popup_chunks` layout instead of the `popup_block` layout to render these widgets into.

# The Exit Popup
We have a way for the user to view their already entered key-value pairs, and we have a way for the user to enter new ones. The last screen we need to create, is the exit/confirmation screen.

In this screen, we are asking the user if they want to output the key-value pairs they have entered in the `stdout` pipe, or close without outputting anything. 
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:exit_screen}}
```
The only thing in this part that we havn't done before, is use the `Clear` widget. This is a special widget that does what the name suggests - it clears everything in the space it is rendered. In this case, it clears all of the menu that was pre-rendered behind it. 


# Helper Functions
Finally, we will implement the `centered_rect` helper function that is referenced above. This code is adapted from the [popup example](https://github.com/ratatui-org/ratatui/blob/main/examples/popup.rs) found in the official repo.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:centered_rect}}
```
