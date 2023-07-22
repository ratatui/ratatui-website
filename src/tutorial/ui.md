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
(Find more information on the different types of constraints [Here TODO AFTER HOW-TOS])
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

## Our layout 
Now that we have our `Frame`, we can actually begin drawing widgets onto it. 
We will begin by creating out layout.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:ui_layout}}
```

The variable `chunks` now contains a length 3 array of `Rect` objects that contain the top left corner of their space, and their size. We will use these later, after we prepare our widgets.

## The title
The title is an important piece for any application. It helps the user understand what they can do and where they are.
To create our title, we are going to use a `Paragraph` widget (which is used to display only text), and we are going to tell that `Paragraph` we want a border all around it by giving it a `Block` with borders enabled. (See [TODO] and [TODO] for more information about `Block` and `Paragraph`).

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:title_paragraph}}
```

In this code, the first thing we do, is create a `Block` with all borders enabled, and the default style.
Next, we created a paragraph widget with the text "Create New Json" styled green. (See [TODO] for more information about creating paragraphs and [TODO] for styling text)
Finally, we call `render_widget` on our `Frame`, and give it the widget we want to render it, and the `Rect` representing where it needs to go and what size it should be. (this is the way all widgets are drawn)

## The list of exist pairs.
We would also like the user to be able to see any key-value pairs that they have already entered. For this, we will be using another widget, the `List`. The list is what it sounds like - it creates a new line of text for each `ListItem`, and it supports passing in a state so you can implement selecting items on the list. We will not be implementing selection, as we simply want the user to be able to see what they have already entered.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/ui.rs:key_value_list}}
```
For more information on Line, Span, and Style see [TODO]
In this piece of the function, we create a vector of `ListItem`s, and populate it with styled and formatted key-value pairs. Finally, we create the `List` widget, and render it. 
