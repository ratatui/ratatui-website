# Alternate Screen

The alternate screen is a separate buffer that some terminals provide, distinct from the main
screen. When activated, the terminal will display the alternate screen, hiding the current content
of the main screen. Applications can write to this screen as if it were the regular terminal
display, but when the application exits, the terminal will switch back to the main screen, and the
contents of the alternate screen will be cleared. This is useful for applications like text editors
or terminal games that want to use the full terminal window without disrupting the command line or
other terminal content.

This creates a seamless transition between the application and the regular terminal session, as the
content displayed before launching the application will reappear after the application exits.

Take this "hello world" program below. If we remove the
`crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)` (and the
corresponding `LeaveAlternateScreen`), you can see how the program behaves differently.

```diff
  use ratatui::{prelude::*, widgets::*};

  fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
-   crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
      terminal.draw(|f| {
        let rects = Layout::default()
          .direction(Direction::Vertical)
          .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
          .split(f.size());
        f.render_widget(Paragraph::new("Hello World! (press 'q' to quit)"), rects[1]);
      })?;

      if crossterm::event::poll(std::time::Duration::from_millis(250))? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
          if key.code == crossterm::event::KeyCode::Char('q') {
            break;
          }
        }
      }
    }

-   crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
  }
```

| With `EnterAlternateScreen`                                                                               | With no `EnterAlternateScreen`                                                                            |
| --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| ![](https://user-images.githubusercontent.com/1813121/272107438-2057ca69-0405-40fe-bc81-f123b05efbde.gif) | ![](https://user-images.githubusercontent.com/1813121/272107285-82dda0f7-b197-4f9b-b57e-9e0f5c025da0.gif) |

Try running this code on your own to experiment with `EnterAlternateScreen` and
`LeaveAlternateScreen`.

Note that not all terminal emulators support the alternate screen, and even those that do may handle
it differently. As a result, the behavior may vary depending on the backend being used. Always
consult the specific backend’s documentation to understand how it implements the alternate screen.
