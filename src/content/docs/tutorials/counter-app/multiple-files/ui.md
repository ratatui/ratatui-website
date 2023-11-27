---
title: ui.rs
---

Previously we were rendering a `Paragraph` with no styling.

Let's make some improvements:

1. Add a `Block` with a rounded border and the title `"Counter App"`.
2. Make everything in the Paragraph have a foreground color of `Color::Yellow`

This is what our code will now look like:

```rust
{{#include @code/ratatui-counter-app/src/ui.rs}}
```

Keep in mind it won't render until we have written the code for `tui::Frame`

When rendered, this is what the UI will look like:

![Counter app demo](https://user-images.githubusercontent.com/1813121/263155937-d8a8b6f6-97f4-4839-b855-ffd0249c2ae0.png)
