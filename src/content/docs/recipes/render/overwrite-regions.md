---
title: "Popups (overwrite regions)"
sidebar:
  order: 3
  label: Popups (overwrite regions)
---

:::tip[TLDR]

Use the [`Clear`] widget to clear areas of the screen to avoid style and symbols from leaking from
previously rendered widgets.

:::

Ratatui renders text in the order that the application writes to the buffer. This means that earlier
instructions will be overwritten by later ones. However, it's important to note that widgets do not
always clear every cell in the area that they are rendering to. This may cause symbols and styles
that were previously rendered to the buffer to "bleed" through into the cells that are rendered on
top of those cells.

The following code exhibits this problem:

```rust
{{#include @code/recipes/how-to-overwrite-regions/src/bin/problem.rs:imports}}

// -- snip --

{{#include @code/recipes/how-to-overwrite-regions/src/bin/problem.rs:ui}}
```

![problem](https://github.com/ratatui/ratatui-website/assets/381361/a32bd6e2-9704-4054-b41d-a34715fc217f)

Notice that the background color (black in this case), the italics, and the lorem ipsum background
text show through the popup.

This problem is easy to prevent by rendering a [`Clear`] widget prior to rendering the main popup.
Here is an example of how to use this technique to create a `Popup` widget:

[`Clear`]: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html

```rust
{{#include @code/recipes/how-to-overwrite-regions/src/bin/solution.rs:imports}}

{{#include @code/recipes/how-to-overwrite-regions/src/bin/solution.rs:popup}}
```

We can use the new `Popup` widget with the following code:

```rust
{{#include @code/recipes/how-to-overwrite-regions/src/bin/solution.rs:solution}}
```

Which results in the following:

![demo](https://github.com/ratatui/ratatui-website/assets/381361/39e92dad-8127-4588-8361-45d2f95abf32)

Notice that the background is set to the default background and there are no italics or symbols from
the background text.

Full source for this article is available at
<https://github.com/ratatui/ratatui-website/tree/main/code/recipes/how-to-overwrite-regions>
