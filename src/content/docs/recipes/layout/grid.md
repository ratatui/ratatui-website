---
title: Grid Layout
sidebar:
  order: 1
---

## Problem

You want to create a grid layout for your TUI application, where widgets are arranged in a grid-like
structure.

## Solution

To create a grid layout, you can use the `Layout` struct to define the horizontal and vertical
constraints of the rows and columns. Combine these constraints with iterator methods to create a
grid layout.

## Example

Given the following grid struct:

```rust
{{ #include @code/recipes/how-to-misc/src/grid.rs:grid }}
```

With the following render method:

```rust
{{ #include @code/recipes/how-to-misc/src/grid.rs:widget }}
```

The output will look like this:

```text
┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐
│Area 01│ │Area 02│ │Area 03│ │Area 04│
└───────┘ └───────┘ └───────┘ └───────┘

┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐
│Area 05│ │Area 06│ │Area 07│ │Area 08│
└───────┘ └───────┘ └───────┘ └───────┘

┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐
│Area 09│ │Area 10│ │Area 11│ │Area 12│
└───────┘ └───────┘ └───────┘ └───────┘

┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐
│Area 13│ │Area 14│ │Area 15│ │Area 16│
└───────┘ └───────┘ └───────┘ └───────┘
```

In Ratatui 0.30, we introduce a few [new methods on Rect], which removes the need to bind rows to
satisfy the borrow checker, and simplifies this to a single line of code:

[new methods on Rect]: https://github.com/ratatui/ratatui/pull/1909

```rust
let cells = area.layout_vec(&vertical).iter().flat_map(|row| row.layout_vec(&horizontal));
```
