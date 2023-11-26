---
title: "How to: Display Text"
---

This page covers how text displaying works. It will cover `Span`, `Line`, and `Text`, and how these
can be created, styled, displayed, altered, and such.

## `Span`

A `Span` is a styled segment of text. You can think of it as a substring with its own unique style.
It is the most basic unit of displaying text in `ratatui`.

The examples below assume the following imports:

```rust
use ratatui::{prelude::*, widgets::*};
pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;
```

A `Span` consists of "content" and a "style" for the content. And a `Span` can be created in a few
different ways.

1. using `Span::raw`:

   ```rust
   fn ui(_app: &App, f: &mut Frame<'_>) {
       let span = Span::raw("This is text that is not styled");
       // --snip--
   }
   ```

2. using `Span::styled`:

   ```rust
   fn ui(_app: &App, f: &mut Frame<'_>) {
       let span = Span::styled("This is text that will be yellow", Style::default().fg(Color::Yellow));
       // --snip--
   }
   ```

3. using the `Stylize` trait:

   ```rust
   fn ui(_app: &App, f: &mut Frame<'_>) {
       let span = "This is text that will be yellow".yellow();
       // --snip--
   }
   ```

A `Span` is the basic building block for any styled text, and can be used anywhere text is
displayed.

## `Line`

The next building block that we are going to talk about is a `Line`. A `Line` represents a cluster
of graphemes, where each unit in the cluster can have its own style. You can think of an instance of
the `Line` struct as essentially a collection of `Span` objects, i.e. `Vec<Span>`.

Since each `Line` struct consists of multiple `Span` objects, this allows for varied styling in a
row of words, phrases or sentences.

```rust
fn ui(_: &App, f: &mut Frame<'_>) {
    let line = Line::from(vec![
        "hello".red(),
        " ".into(),
        "world".red().bold()
    ]);
    // --snip--
}
```

A `Line` can be constructed directly from content, where the content is `Into<Cow<'a, &str>>`.

```rust
fn ui(_: &App, f: &mut Frame<'_>) {
    let line = Line::from("hello world");
    // --snip--
}
```

You can even style a full line directly:

```rust
fn ui(_: &App, f: &mut Frame<'_>) {
    let line = Line::styled("hello world", Style::default().fg(Color::Yellow));
    // --snip--
}
```

And you can use the `Stylize` trait on the line directly by using `into()`:

```rust
fn ui(_: &App, f: &mut Frame<'_>) {
    let line: Line = "hello world".yellow().into();
    // --snip--
}
```

## `Text`

`Text` is the final building block of outputting text. A `Text` object represents a collection of
`Line`s.

Most widgets accept content that can be converted to `Text`.

```rust
fn ui(_: &App, f: &mut Frame<'_>) {
    let span1 = "hello".red();
    let span2 = "world".red().bold();
    let line = Line::from(vec![span1, " ".into(), span2]);
    let text = Text::from(line);
    f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL)), f.size());
}
```

Here's an HTML representation of what you'd get in the terminal:

<div style="border: 1px solid black; display: inline-block; padding: 5px;">
    <span style="color: red;">hello</span>
    <span style="color: red; font-weight: bold;">world</span>
</div>

Often code like the one above can be simplified:

```rust
fn ui(_: &App, f: &mut Frame<'_>) {
    let line: Line = vec![
        "hello".red(),
        " ".into(),
        "world".red().bold()
    ].into();
    f.render_widget(Paragraph::new(line).block(Block::default().borders(Borders::ALL)), f.size());
}
```

This is because in this case, Rust is able to infer the types and convert them into appropriately.

`Text` instances can be created using the `raw` or `styled` constructors too.

Something that you might find yourself doing pretty often for a `Paragraph` is wanting to have
multiple lines styled differently. This is one way you might go about that:

```rust
fn ui(_: &App, f: &mut Frame<'_>) {
    let text = vec![
        "hello world 1".into(),
        "hello world 2".blue().into(),
        Line::from(vec!["hello".green(), " ".into(), "world".green().bold(), "3".into()]),
    ]
    .into();
    f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL)), f.size());
}
```

<div style="border: 1px solid black; display: inline-block; padding: 5px;">
    <p>
        hello world 1
    </p>
    <p>
        <span style="color: blue;">hello world 2</span>
    </p>
    <p>
        <span style="color: green;">hello</span>
        <span style="color: green; font-weight: bold;">world</span> 3
    </p>
</div>

We will talk more about styling in the next section.
