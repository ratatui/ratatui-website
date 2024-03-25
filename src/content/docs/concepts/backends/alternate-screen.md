---
title: Alternate Screen
sidebar:
  order: 3
---

The alternate screen is a separate buffer that some terminals provide, distinct from the main
screen. When activated, the terminal will display the alternate screen, hiding the current content
of the main screen. Applications can write to this screen as if it were the regular terminal
display, but when the application exits, the terminal will switch back to the main screen, and the
contents of the alternate screen will be cleared. This is useful for applications like text editors
or terminal games that want to use the full terminal window without disrupting the command line or
other terminal content.

This creates a seamless transition between the application and the regular terminal session, as the
content displayed before launching the application will reappear after the application exits.

Take this "hello world" program below. If we run it with and without the
`std::io::stderr().execute(EnterAlternateScreen)?` (and the corresponding `LeaveAlternateScreen`),
you can see how the program behaves differently.

```rust
use std::{ // hide_line
  io::{stderr, Result}, // hide_line
  thread::sleep, // hide_line
  time::Duration, // hide_line
}; // hide_line
// hide_line
use crossterm::{ // hide_line
  terminal::{EnterAlternateScreen, LeaveAlternateScreen}, // hide_line
  ExecutableCommand, // hide_line
}; // hide_line
use ratatui::{prelude::*, widgets::*}; // hide_line
// hide_line
fn main() -> Result<()> { // hide_line
  let should_enter_alternate_screen = std::env::args().nth(1).unwrap().parse::<bool>().unwrap(); // hide_line
  if should_enter_alternate_screen { // hide_line
  stderr().execute(EnterAlternateScreen)?; // remove this line
  } // hide_line

  let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

  terminal.draw(|f| {
    f.render_widget(Paragraph::new("Hello World!"), Rect::new(10, 20, 20, 1));
  })?;
  sleep(Duration::from_secs(2));

  if should_enter_alternate_screen { // hide_line
  stderr().execute(LeaveAlternateScreen)?; // remove this line
  } // hide_line
  Ok(()) // hide_line
} // hide_line
```

<!--
Output ./demo.gif

Set FontSize 18
Set Width 1200
Set Height 800
Set Theme "Catppuccin Mocha"

Type "# WITH Alternate Screen"
Enter
Type "# Cursor is here before program starts"
Enter

Sleep 5s

Type "cargo run -- true"
Enter
Sleep 5s

Type "# Cursor is here after program completes"
Enter

Sleep 5s

# Type "reset"

# Enter

# Sleep 2s

Type "# WITHOUT Alternate Screen"
Enter
Type "# Cursor is here before program starts"
Enter

Sleep 5s

Type "cargo run -- false"
Enter
Sleep 5s

Type "# Cursor is here after program completes"
Enter

Sleep 5s
-->

![](https://user-images.githubusercontent.com/1813121/272299743-f666980f-93b8-40d4-a979-1fce26d0f84a.gif)

Try running this code on your own and experiment with `EnterAlternateScreen` and
`LeaveAlternateScreen`.

Note that not all terminal emulators support the alternate screen, and even those that do may handle
it differently. As a result, the behavior may vary depending on the backend being used. Always
consult the specific backend’s documentation to understand how it implements the alternate screen.
