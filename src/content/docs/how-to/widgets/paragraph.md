---
title: Paragraph
sidebar:
  order: 2
---

The `Paragraph` widget provides a way to display text content in your terminal user interface. It
allows not only plain text display but also handling text wrapping, alignment, and styling. This
page will delve deeper into the functionality of the `Paragraph` widget.

## Usage

```rust
let p = Paragraph::new("Hello, World!");
f.render_widget(p, chunks[0]);
```

## Styling and Borders

You can also apply styles to your text and wrap it with a border:

```rust
let p = Paragraph::new("Hello, World!")
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Title")
            .border_type(BorderType::Rounded)
    );
f.render_widget(p, chunks[0]);
```

## Wrapping

The `Paragraph` widget will wrap the content based on the available width in its containing block.
You can also control the wrapping behavior using the `wrap` method:

```rust
let p = Paragraph::new("A very long text that might not fit the container...")
    .wrap(Wrap { trim: true });
f.render_widget(p, chunks[0]);
```

Setting `trim` to `true` will ensure that trailing whitespaces at the end of each line are removed.

## Alignment

```rust
let p = Paragraph::new("Centered Text")
    .alignment(Alignment::Center);
f.render_widget(p, chunks[0]);
```

## Styled Text

`Paragraph` supports rich text through `Span`, `Line`, and `Text`:

```rust
let lines = vec![];
lines.push(Line::from(vec![
    Span::styled("Hello ", Style::default().fg(Color::Yellow)),
    Span::styled("World", Style::default().fg(Color::Blue).bg(Color::White)),
]));
lines.push(Line::from(vec![
    Span::styled("Goodbye ", Style::default().fg(Color::Yellow)),
    Span::styled("World", Style::default().fg(Color::Blue).bg(Color::White)),
]));
let text = Text::from(lines);
let p = Paragraph::new(text);
f.render_widget(p, chunks[0]);
```

## Scrolling

For long content, `Paragraph` supports scrolling:

```rust
let mut p = Paragraph::new("Lorem ipsum ...")
    .scroll((1, 0));  // Scroll down by one line
f.render_widget(p, chunks[0]);
```
