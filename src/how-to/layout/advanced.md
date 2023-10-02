# Layout Advanced

## Nested layouts

It's often required to display widgets within widgets. Think of how modern UIs have panels inside
panels, each containing its own content. You can achieve this through nested layouts.

```rust
let main_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
        .as_ref(),
    )
    .split(f.size());

let top_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ]
        .as_ref(),
    )
    .split(main_chunks[0]);

let bottom_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ]
        .as_ref(),
    )
    .split(main_chunks[1]);

```

Here's what the layout looks like:

```text
┌──────────────────────────────────┐┌──────────────┐
│                                  ││              │
│                                  ││              │
│                                  ││              │
│                                  ││              │
│                                  ││              │
│                                  ││              │
└──────────────────────────────────┘└──────────────┘
┌───────────────────┐┌─────────────────────────────┐
│                   ││                             │
│                   ││                             │
│                   ││                             │
│                   ││                             │
│                   ││                             │
└───────────────────┘└─────────────────────────────┘
```

Here's the same with labels and margins:

```rust
      let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size().inner(&Margin { horizontal: 1, vertical: 1 }));
      let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(main_chunks[0].inner(&Margin { horizontal: 1, vertical: 1 }));
      let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(main_chunks[1].inner(&Margin { horizontal: 1, vertical: 1 }));

      f.render_widget(
          Block::default()
              .borders(Borders::all())
              .border_type(BorderType::Rounded)
              .title("Main"),
          f.size(),
      );
      f.render_widget(
          Block::default()
              .borders(Borders::all())
              .border_type(BorderType::Thick)
              .title("Top"),
          main_chunks[0],
      );
      f.render_widget(
          Block::default()
              .borders(Borders::all())
              .border_type(BorderType::Double)
              .title("Top Left"),
          top_chunks[0],
      );
      f.render_widget(
          Block::default()
              .borders(Borders::all())
              .title("Top Right"),
          top_chunks[1]
      );
      f.render_widget(
          Block::default()
              .borders(Borders::all())
              .border_type(BorderType::Thick)
              .title("Bottom"),
          main_chunks[1],
      );
      f.render_widget(
          Block::default()
              .borders(Borders::all())
              .title("Bottom Left"),
          bottom_chunks[0]
      );
      f.render_widget(
        Block::default()
              .borders(Borders::all())
              .border_type(BorderType::Double)
              .title("Bottom Right"),
        bottom_chunks[1],
      );
```

```text
╭Main──────────────────────────────────────────────╮
│┏Top━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓│
│┃╔Top Left════════════════════════╗┌Top Right───┐┃│
│┃║                                ║│            │┃│
│┃║                                ║│            │┃│
│┃║                                ║│            │┃│
│┃╚════════════════════════════════╝└────────────┘┃│
│┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛│
│┏Bottom━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓│
│┃┌Bottom Left──────┐╔Bottom Right═══════════════╗┃│
│┃│                 │║                           ║┃│
│┃│                 │║                           ║┃│
│┃└─────────────────┘╚═══════════════════════════╝┃│
│┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛│
╰──────────────────────────────────────────────────╯
```

## Dynamic layouts

With real-world applications, the content can often be dynamic. For example, a chat application may
need to resize the chat input area based on the number of incoming messages. To achieve this, you
can generate layouts dynamically:

```rust
fn get_layout_based_on_messages(msg_count: usize, f: &Frame) -> Vec<Rect> {
    let msg_percentage = if msg_count > 50 { 80 } else { 50 };

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(msg_percentage),
                Constraint::Percentage(100 - msg_percentage),
            ]
            .as_ref(),
        )
        .split(f.size())
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
    .constraints(
        [
            Constraint::Percentage(current_percentage),
            Constraint::Percentage(100 - current_percentage),
        ]
        .as_ref(),
    )
    .split(f.size());

```
