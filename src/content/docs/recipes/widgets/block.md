---
title: Block
sidebar:
  order: 1
---

The `Block` widget serves as a foundational building block for structuring and framing other
widgets. It's essentially a container that can have borders, a title, and other styling elements to
enhance the aesthetics and structure of your terminal interface. This page provides an in-depth
exploration of the `Block` widget.

## Basic Usage

The simplest use case for a `Block` is to create a container with borders:

```rust
let b = Block::default()
    .borders(Borders::ALL);
f.render_widget(b, chunks[0]);
```

## Titles

A common use case for Block is to give a section of the UI a title or a label:

```rust
let b = Block::default()
    .title("Header")
    .borders(Borders::ALL);
f.render_widget(b, chunks[0]);
```

You can also use the `Line` struct for better positioning or multiple titles.

```rust
let b = Block::default()
    .title(Line::from("Left Title").alignment(Alignment::Left))
    .title(Line::from("Middle Title").alignment(Alignment::Center))
    .title(Line::from("Right Title").alignment(Alignment::Right))
    .borders(Borders::ALL);
f.render_widget(b, chunks[0]);
```

## Border style

Block provides flexibility in both the borders style and type:

```rust
let b = Block::default()
    .title("Styled Header")
    .border_style(Style::default().fg(Color::Magenta))
    .border_type(BorderType::Rounded)
    .borders(Borders::ALL);
f.render_widget(b, chunks[0]);
```
