---
title: "How to: Style Text"
---

Styling enhances user experience by adding colors, emphasis, and other visual aids. In `ratatui`,
the primary tool for this is the `ratatui::style::Style` struct.

`ratatui::style::Style` provides a set of methods to apply styling attributes to your text. These
styles can then be applied to various text structures like `Text`, `Span`, and `Line` (as well as
other non text structures).

Common styling attributes include:

- Foreground and Background Colors (`fg` and `bg`)
- Modifiers (like `bold`, `italic`, and `underline`)

1. Basic Color Styling

   Setting the foreground (text color) and background:

   ```rust
   let styled_text = Span::styled(
       "Hello, Ratatui!",
       Style::default().fg(Color::Red).bg(Color::Yellow)
   );
   ```

2. Using `Modifiers`

   Making text bold or italic:

   ```rust
   let bold_text = Span::styled(
       "This is bold",
       Style::default().add_modifier(Modifier::BOLD)
   );

   let italic_text = Span::styled(
       "This is italic",
       Style::default().add_modifier(Modifier::ITALIC)
   );
   ```

   You can also combine multiple modifiers:

   ```rust
   let bold_italic_text = Span::styled(
       "This is bold and italic",
       Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC)
   );
   ```

3. Styling within a Line

   You can mix and match different styled spans within a single line:

   ```rust
   let mixed_line = Line::from(vec![
       Span::styled("This is mixed", Style::default().fg(Color::Green)),
       Span::styled("styling", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
       Span::from("!"),
   ]);
   ```

This is what it would look like if you rendered a `Paragraph` with different styles for each line:

```rust
fn ui(_: &App, f: &mut Frame) {
  let styled_text = Span::styled("Hello, Ratatui!", Style::default().fg(Color::Red).bg(Color::Yellow));
  let bold_text = Span::styled("This is bold", Style::default().add_modifier(Modifier::BOLD));
  let italic_text = Span::styled("This is italic", Style::default().add_modifier(Modifier::ITALIC));
  let bold_italic_text =
    Span::styled("This is bold and italic", Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC));
  let mixed_line = vec![
    Span::styled("This is mixed", Style::default().fg(Color::Green)),
    Span::styled("styling", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    Span::from("!"),
  ];
  let text: Vec<Line<'_>> =
    vec![styled_text.into(), bold_text.into(), italic_text.into(), bold_italic_text.into(), mixed_line.into()];
  f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL)), f.size());
}
```

Here's the HTML representation of the above styling:

<div style="border: 1px solid black; display: inline-block; padding: 5px;">
    <p style="color: red; background-color: yellow;">Hello, Ratatui!</p>
    <p style="font-weight: bold;">This is bold</p>
    <p style="font-style: italic;">This is italic</p>
    <p style="font-weight: bold; font-style: italic;">This is bold and italic</p>
    <p>
        <span style="color: green;">This is mixed</span>
        <span style="color: red; font-weight: bold;">styling</span>
        !
    </p>
</div>

:::tip

You can also create instances of `Color` from a string:

```rust
use std::str::FromStr;

let color: Color = Color::from_str("blue").unwrap();
assert_eq!(color, Color::Blue);

let color: Color = Color::from_str("#FF0000").unwrap();
assert_eq!(color, Color::Rgb(255, 0, 0));

let color: Color = Color::from_str("10").unwrap();
assert_eq!(color, Color::Indexed(10));
```

:::

You can read more about the
[`Color` enum](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html) and
[`Modifier`](https://docs.rs/ratatui/latest/ratatui/style/struct.Modifier.html) in the reference
documentation online.
