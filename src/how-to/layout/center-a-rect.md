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
    .constraints(
      [
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
      ]
      .as_ref(),
    )
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
      [
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
      ]
      .as_ref(),
    )
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
