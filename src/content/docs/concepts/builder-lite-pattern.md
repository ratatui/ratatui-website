---
title: Builder Lite Pattern
---

In Ratatui, most widgets (and some other objects) use the [Builder Lite] pattern to set fields. This
allows the object to be created in a single shot with methods that setup how the widget will be
displayed, without having to store the widget in a variable and mutate it.

The builder lite pattern consumes the `self` parameter of each method and returns a value with the
updated field. An example of this from Paragraph (and any other widget that supports being
automatically wrapped in a block):

```rust
#[must_use]
pub fn block(mut self, block: Block<'a>) -> Self {
    self.block = Some(block);
    self
}
```

Which you might call like:

```rust
let paragraph = Paragraph::new("foobar").block(Block::bordered())
```

If you've reached this page after seeing an error or warning in your app's compilation, then it's
likely that you are calling the setter methods against an object, but not storing or using the
result. This will have no effect on the actual display of the widget and is usually a mistake.

E.g. the following code:

```rust
let text = Text::raw("wrong");
text.centered();
```

Should be replaced with:

```rust
let text = Text::raw("right").centered();
```

Or in situations where you want to reuse a widget's setup more than once:

```rust
let text = Text::raw("right");
let centered_text = text.clone().centered();
let bold_text = text.bold();
```

[Builder Lite]: https://matklad.github.io/2022/05/29/builder-lite.html
