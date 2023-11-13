# Center a `Rect`

You can use a `Vertical` layout followed by a `Horizontal` layout to get a centered `Rect`.

````rust
/// # Usage
///
/// ```rust
/// let rect = centered_rect(f.size(), 50, 50);
/// ```
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
````

Then you can use it to draw any widget like this:

```rust
terminal.draw(|f| {
    f.render_widget(Block::default().borders(Borders::all()).title("Main"), centered_rect(f.size(), 35, 35));
})?;
```

```text






                    ┌Main────────────────────────────────┐
                    │                                    │
                    │                                    │
                    │                                    │
                    │                                    │
                    │                                    │
                    │                                    │
                    │                                    │
                    │                                    │
                    └────────────────────────────────────┘










```

A common use case for this feature is to create a popup style dialog block. For this, typically,
you'll want to `Clear` the popup area before rendering your content to it.
The following is an example of how you might do that:

```rust
terminal.draw(|f| {
    let popup_area = centered_rect(f.size(), 35, 35);
    f.render_widget(Clear, popup_area);
    f.render_widget(Block::default().borders(Borders::all()).title("Main"), popup_area);
})?;
```
