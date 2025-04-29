---
title: "Centering a `Rect`"
sidebar:
  order: 2
  label: Center a Rect
---

You can use the `.centered()` method to get a centered `Rect`

```rust title=layout.rs collapse={1-13}
{{ #include @code/recipes/how-to-misc/src/layout.rs:center }}
```

You can use this method to draw any widget centered on the containing area.

```rust
{{ #include @code/recipes/how-to-misc/src/layout.rs:render }}
```

A common use case for this feature is to create a popup style dialog block. For this, typically,
you'll want to `Clear` the popup area before rendering your content to it. The following is an
example of how you might do that:

```rust
{{ #include @code/recipes/how-to-misc/src/layout.rs:render_popup }}
```

:::note

There is no method for vertically aligning text within an area yet. We recommend prewrap the text
using the [textwrap crate] and then use the line count to work out where to render the text.

:::

[textwrap crate]: https://crates.io/crates/textwrap

Full code for this recipe is available in the website repo at:
<https://github.com/ratatui/ratatui-website/blob/main/code/recipes/src/layout.rs>

## See also

There are several third party widget libraries for making popups easy to use:

- [tui-popup](https://crates.io/crates/tui-popup)
- [tui-confirm-dialog](https://crates.io/crates/tui-confirm-dialog)
