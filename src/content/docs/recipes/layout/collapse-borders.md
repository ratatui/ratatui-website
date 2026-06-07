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

Starting with Ratatui 0.30, collapsing borders has become much easier thanks to the new
`merge_borders` method and `Spacing::Overlap`. The recipe is simple:

1.  Import `Spacing` and `MergeStrategy`.

    ```rust ins={2-3}
    {{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:imports}}
    ```

2.  Use `Spacing::Overlap(1)` in your layout to make borders overlap.

    ```rust
    {{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:layout}}
    ```

3.  Add `.merge_borders(MergeStrategy::Exact)` to your blocks to automatically merge borders (see
    [`MergeStrategy` documentation](https://docs.rs/ratatui/latest/ratatui/symbols/merge/enum.MergeStrategy.html#variants)
    for details about the different strategies).

    ```rust
    {{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs:blocks}}
    ```

Setting `merge_borders` to `MergeStrategy::Exact` or `MergeStrategy::Fuzzy` automatically handles
all the complex border joining logic. The `Spacing::Overlap(1)` ensures that adjacent borders occupy
the same space, allowing them to be merged.

:::tip

This new approach in Ratatui 0.30 replaces the complex manual border management that was required in
earlier versions. If you're using an older version of Ratatui, you'll need to use custom border sets
and selective border rendering as described in the previous version of this recipe.

:::

The full code for this example is available at
<https://github.com/ratatui/ratatui-website/blob/main/code/recipes/how-to-collapse-borders>

```rust collapsed title="collapse-borders.rs"
{{#include @code/recipes/how-to-collapse-borders/src/bin/solution.rs}}
```
