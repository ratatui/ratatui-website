---
title: Create custom widgets
sidebar:
  order: 3
---

While Ratatui offers a rich set of pre-built widgets, there may be scenarios where you require a
unique component tailored to specific needs. In such cases, creating a custom widget becomes
invaluable. This page will guide you through the process of designing and implementing custom
widgets.

## `Widget` trait

At the core of creating a custom widget is the `Widget` trait. Any struct that implements this trait
can be rendered using the framework's drawing capabilities.

```rust

pub struct MyWidget {
    // Custom widget properties
    content: String,
}

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Rendering logic goes here
    }
}
```

The `render` method must draw into the current `Buffer`. There are a number of methods implemented
on `Buffer`.

```rust
impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), &self.content, Style::default().fg(Color::Green));
    }
}
```

For a given state, the `Widget` trait implements how that struct should be rendered.

```rust
pub struct Button {
    label: String,
    is_pressed: bool,
    style: Style,
    pressed_style: Option<Style>,
}

impl Widget for Button {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.is_pressed {
            self.pressed_style.unwrap_or_else(|| Style::default().fg(Color::Blue))
        } else {
            self.style
        };
        buf.set_string(area.left(), area.top(), &self.label, style);
    }
}
```

Ratatui also has a `StatefulWidget`. This is essentially a widget that can "remember" information
between two draw calls. This is essential when you have interactive UI components, like lists, where
you might need to remember which item was selected or how much the user has scrolled.

Here's a breakdown of the trait:

```rust
pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
```

- `type State`: This represents the type of the state that this widget will use to remember details
  between draw calls.
- `fn render(...)`: This method is responsible for drawing the widget on the terminal. Notably, it
  also receives a mutable reference to the state, allowing you to read from and modify the state as
  needed.
