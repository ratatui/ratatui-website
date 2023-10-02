# Layout Basics

Here's the "hello world" example again:

```rust
pub fn render(app: &mut App, f: &mut Frame) {
  f.render_widget(
    Paragraph::new("Hello World!")
      .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)),
    f.size()
  )
}
```

Here's what the docs say for `f.size()`:

```raw
ratatui::terminal::Frame

pub fn size(&self) -> Rect
────────────────────────────────────────────────────
Frame size, guaranteed not to change when rendering.
```

`f.size()` returns a `Rect` struct. A `Rect` has the following `struct` definition:

```rust
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
```

That is to say, they have a `x` and `y` positional coordinates and `width` and `height` dimensional
values.

The coordinate system in `ratatui` (and in terminals in general) starts at the top left of the
terminal or container widget. This point represents `(0,0)`.

```svgbob
( 0, 0 ) --------------> x

   |
   |
   |
   |
   v

   y
```

Here's the "hello world" example from above rendered:

```raw
╭───────────────────────────────────╮
│Hello World!                       │
│                                   │
│                                   │
╰───────────────────────────────────╯
```

What if hypothetically we wanted to render this instead:

```raw
╭────────────────╮╭─────────────────╮
│Hello World!    ││Hello World!     │
│                ││                 │
│                ││                 │
╰────────────────╯╰─────────────────╯
```

We _could_ integer divide the `width` by 2, account of the borders calculate the `x` position for
the second paragraph but that is cumbersome and error prone.

Now, that's where layouts come in.

```rust
let rects = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
        .as_ref(),
    )
    .split(f.size());
```

Here we created a layout and added two "constraints". The constraints determine the size of the
resulting `Rect`s. Calling `split` on a `Layout` splits the layout based on the constraints.

That is, `rects` behaves as a `Vec<Rect>`, whose length always matches the number of constraints.

So for the example above, we might want to do something like this:

```rust
pub fn render(app: &mut App, f: &mut Frame) {
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
        .as_ref(),
    )
    .split(f.size());
  f.render_widget(
    Paragraph::new("Hello World!")
      .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)),
    chunks[0]
  )
  f.render_widget(
    Paragraph::new("Hello World!")
      .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)),
    chunks[1]
  )
}
```

Notice that we used the first `chunk` for the first `Paragraph` and the second `chunk` for the
second `Paragraph`.
