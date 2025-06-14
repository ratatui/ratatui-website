---
title: "Create Dynamic layouts"
sidebar:
  order: 4
  label: Dynamic Layouts
---

With real world applications, the content can often be dynamic. For example, a chat application may
need to resize the chat input area based on the number of incoming messages. To achieve this, you
can generate layouts dynamically:

```rust
fn get_layout_based_on_messages(msg_count: usize, f: &Frame) -> Rc<[Rect]> {
    let msg_percentage = if msg_count > 50 { 80 } else { 50 };

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(msg_percentage),
            Constraint::Percentage(100 - msg_percentage),
        ])
        .split(f.area())
}
```

You can even update the layout based on some user input or command:

```rust
match action {
    Action::IncreaseSize => {
        current_percentage += 5;
        if current_percentage > 95 {
            current_percentage = 95;
        }
    },
    Action::DecreaseSize => {
        current_percentage -= 5;
        if current_percentage < 5 {
            current_percentage = 5;
        }
    },
    _ => {}
}

let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(current_percentage),
        Constraint::Percentage(100 - current_percentage),
    ])
    .split(f.area());

```
