---
title: UI.rs
---

Finally we come to the last piece of the puzzle, and also the hardest part when you are just
starting out creating `ratatui` TUIs --- the UI. We created a very simple UI with just one widget in
the previous tutorial, but here we'll explore some more sophisticated layouts.

:::note

If you have created a UI before, you should know that the UI code can take up much more space than
you think it should, and this is not exception. We will only briefly cover all the functionality
available in `ratatui` and how the core of `ratatui` design works.

There will be links to more resources where they are covered in depth in the following sections.

:::

## Layout basics

Our first step is to grasp how we render widgets onto the terminal.

In essence: Widgets are constructed and then drawn onto the screen using a `Frame`, which is placed
within a specified `Rect`.

Now, envision a scenario where we wish to divide our renderable `Rect` area into three distinct
areas. For this, we can use the `Layout` functionality in `ratatui`.

```rust
{{#include @code/ratatui-json-editor-app/src/ui.rs:ui_layout}}
```

This can be likened to partitioning a large rectangle into smaller sections.

:::tip

For a better understanding of layouts and constraints, refer to the concepts page on
[Layout](/concepts/layout/).

:::

In the example above, you can read the instructions aloud like this:

1. Take the area `f.size()` (which is a rectangle), and cut it into three vertical pieces (making
   horizontal cuts).
2. The first section will be 3 lines tall
3. The second section should never be smaller than one line tall, but can expand if needed.
4. The final section should also be 3 lines tall

For those visual learners, I have the following graphic:

```kroki type=svgbob
+--------------------------------------------+
|          Top segment always                | Constraint::Length  == 3
|          remains 3 lines                   |
+--------------------------------------------+
|                                            |
|                                            |
|      Middle segment maintains a minimum    |
|      height of 1 line, but can expand if   |
|      additional space is present.          | Constraint::Length  > = 1
|                                            |
|                                            |
+--------------------------------------------|
|          Bottom segment is                 | Constraint::Length  == 3
|          consistently 3 lines              |
+--------------------------------------------+
```

Now that we have that out of the way, let us create the TUI for our application.

## The function signature

Our UI function needs two things to successfully create our UI elements. The `Frame` which contains
the size of the terminal at render time (this is important, because it allows us to take resizable
terminals into account), and the application state.

```rust
{{#include @code/ratatui-json-editor-app/src/ui.rs:method_sig}}
```

Before we proceed, let's implement a `centered_rect` helper function. This code is adapted from the
[popup example](https://github.com/ratatui-org/ratatui/blob/main/examples/popup.rs) found in the
official repo.

```rust
{{#include @code/ratatui-json-editor-app/src/ui.rs:centered_rect}}
```

This will be useful for the later subsections.
