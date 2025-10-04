---
title: "Collapse borders in a layout"
sidebar:
  order: 3
  label: Collapse Borders
---

A common layout for applications is to split up the screen into panes, with borders around each
pane. Often this leads to making UIs that look disconnected. E.g., the following layout:

![problem](https://user-images.githubusercontent.com/381361/279935613-01b5083d-dcca-4ee3-981c-38fe700bbfe4.png)

Created by the following code:

```rust
{{#include @code/recipes/how-to-collapse-borders/src/bin/problem.rs:draw}}
```

We can do better though, by collapsing borders. E.g.:

![solution](https://user-images.githubusercontent.com/381361/279935618-3b411b45-1a02-4f4c-af9f-7b68f766023e.png)

The first thing we need to do is work out which borders to collapse. Because in the layout above we
want to connect the bottom right block to the middle vertical border, we're going to need this to be
rendered by the top left and bottom left blocks rather than the right block.

We need to use the symbols module to achieve this so we add this to the imports:

```rust ins={3}
{{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:imports}}
```

Our first change is to the left block where we remove the right border:

```rust
{{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:left_block}}
```

Next, we see that the top left corner of the top right block joins with the top right corner of the
left block, so we need to replace that with a T shape. We also see omit the bottom border as that
will be rendered by the bottom right block. We use a custom [`symbols::border::Set`] to achieve
this.

[`symbols::border::Set`]: https://docs.rs/ratatui/latest/ratatui/symbols/border/struct.Set.html

```rust
{{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:top_right_block}}
```

In the bottom right block, we see that the top right corner joins the left block's right border and
so we need to rend this with a horizontal T shape pointing to the right. We need to do the same for
the top right corner and the bottom left corner.

```rust
{{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:bottom_right_block}}
```

Finally, render the blocks:

```rust
{{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:render}}
```

If we left it here, then we'd be mostly fine, but in small areas we'd notice that the 50/50 split no
longer looks right. This is due to the fact that by default we round up when splitting an odd number
of rows or columns in 2 (e.g. 5 rows => 2.5/2.5 => 3/2). This is fine normally, but when we collapse
borders between blocks, the first block has one extra row (or columns) already as it does not have
the collapsed block. We can easily work around this issue by allocating a small amount of extra
space to the last layout item (e.g. by using 49/51 or 33/33/34).

```rust
{{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:layout}}
```

:::note

If this sounds too complex, we're looking for some help to make this easier in
<https://github.com/ratatui/ratatui/issues/605>.

:::

The full code for this example is available at
<https://github.com/ratatui/ratatui-website/blob/main/code/recipes/how-to-collapse-borders>

```rust collapsed title="collapse-borders.rs"
{{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs}}
```
