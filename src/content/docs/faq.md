---
title: FAQ
---

## Why am I getting duplicate key events on Windows?

A lot of examples out there in the wild might use the following code for sending key presses:

```rust
  CrosstermEvent::Key(e) => tx.send(Event::Key(e)),
```

However, on Windows, when using `Crossterm`, this will send the same `Event::Key(e)` twice; one for
when you press the key, i.e. `KeyEventKind::Press` and one for when you release the key, i.e.
`KeyEventKind::Release`. On `MacOS` and `Linux` only `KeyEventKind::Press` kinds of `key` event is
generated.

To make the code work as expected across all platforms, you can do this instead:

```rust
  CrosstermEvent::Key(key) => {
    if key.kind == KeyEventKind::Press {
      event_tx.send(Event::Key(key)).unwrap();
    }
  },
```

## When should I use `tokio` and `async`/`await`?

`ratatui` isn't a native `async` library. So is it beneficial to use `tokio` or `async`/`await`?

As a user of `rataui`, there really is only one point of interface with the `ratatui` library and
that's the `terminal.draw(|f| ui(f))` functionality (the creation of widgets provided by `ratatui`
typically happens in `ui(f)`). Everything else in your code is your own to do as you wish.

Should `terminal.draw(|f| ui(f))` be `async`? Possibly. Rendering to the terminal buffer is
relatively fast, especially using the double buffer technique that only renders diffs that `ratatui`
uses. Creating of the widgets can also be done quite efficiently.

So one question you may ask is can we make `terminal.draw(|f| ui(f))` `async` ourselves? Yes, we
can. Check out <https://github.com/ratatui-org/ratatui-async-template/tree/v0.1.0> for an example.

The only other part related to `ratatui` that is beneficial to being `async` is reading the key
event inputs from `stdin`, and that can be made `async` with `crossterm`'s event-stream.

So the real question is what other parts of your app require `async` or benefit from being `async`?
If the answer is not much, maybe it is simpler to not use `async` and avoiding `tokio`.

Another way to think about it is, do you think your app would work better with 1 thread like this?

```kroki type=svgbob
 ,-------------.
 |Get Key Event|
 `-----+-------'
       |
       |
 ,-----v------.
 |Update State|
 `-----+------'
       |
       |
   ,---v----.
   | Render |
   `--------'
```

:::note

Even with the above architecture, you can use tokio to spawn tasks during `Update State`, and follow
up on the work done by those tasks in the next iteration.

:::

Or would it work with 3 threads / `tokio` tasks like this:

```kroki type=svgbob
    Render Thread       ┊         Event Thread             ┊     Main Thread
                        ┊                                  ┊
                        ┊      ,------------------.        ┊
                        ┊      |  Get Key Event   |        ┊
                        ┊      `--------+---------'        ┊
                        ┊               |                  ┊
                        ┊     ,---------v-----------.      ┊
                        ┊     | Map Event to Action |      ┊
                        ┊     `---------+-----------'      ┊
                        ┊               |                  ┊
                        ┊  ,------------V--------------.   ┊     ,-------------.
                        ┊  | Send Action on action_tx  |---┊---->| Recv Action |
                        ┊  `---------------------------'   ┊     `-----+-------'
                        ┊                                  ┊           |
,-------------------.   ┊                                  ┊  ,--------v--------.
| Recv on render_rx |<--┊----------------------------------┊--| Dispatch Action |
`--------+----------'   ┊                                  ┊  `--------+--------'
         |              ┊                                  ┊           |
,--------v---------.    ┊                                  ┊  ,--------v---------.
| Render Component |    ┊                                  ┊  |   Update State   |
`------------------'    ┊                                  ┊  `------------------'
```

In your `main` thread or `tokio` task, do you expect to be spawning more `tokio` tasks? How many
more tasks do you plan to be spawning?

The former can be done without any `async` code and the latter is the approach showcased in
[`ratatui-async-template#v1.0`](https://github.com/ratatui-org/ratatui-async-template/tree/v0.1.0)
with `tokio`.

The most recent version of the `ratatui-async-template` uses this architecture instead with tokio:

```kroki type=svgbob
       Event Thread             ┊     Main Thread
                                ┊
    ,------------------.        ┊
    |  Get Key Event   |        ┊
    `--------+---------'        ┊
             |                  ┊
,------------V--------------.   ┊     ,-------------.
| Send Event on event_tx    |---┊---->| Recv Event  |
`---------------------------'   ┊     `-----+-------'
                                ┊           |
                                ┊  ,--------v------------.
                                ┊  | Map Event to Action |
                                ┊  `--------+-----+------'
                                ┊           |     |
                                ┊         Tick    '----------.
                                ┊           |                |
                                ┊  ,--------v---------.      |
                                ┊  |   Update State   |    Render
                                ┊  `------------------'      |
                                ┊                            |
                                ┊                   ,--------v---------.
                                ┊                   | Render Component |
                                ┊                   `------------------'
```

## tui.rs history

This project was forked from [`tui-rs`](https://github.com/fdehau/tui-rs/) in February 2023, with
the [blessing of the original author](https://github.com/fdehau/tui-rs/issues/654), Florian Dehau
([@fdehau](https://github.com/fdehau)).

The original repository contains all the issues, PRs and discussion that were raised originally, and
it is useful to refer to when contributing code, documentation, or issues with Ratatui.

We imported all the PRs from the original repository and implemented many of the smaller ones and
made notes on the leftovers. These are marked as draft PRs and labelled as
[imported from tui](https://github.com/ratatui-org/ratatui/pulls?q=is%3Apr+is%3Aopen+label%3A%22imported+from+tui%22).
We have documented the current state of those PRs, and anyone is welcome to pick them up and
continue the work on them.

We have not imported all issues opened on the previous repository. For that reason, anyone wanting
to **work on or discuss** an issue will have to follow the following workflow:

- Recreate the issue
- Start by referencing the **original issue**:
  `Referencing issue #[<issue number>](<original issue link>)`
- Then, paste the original issues **opening** text

You can then resume the conversation by replying to the new issue you have created.

## What is the difference between a library and a framework?

The terms library and framework are often used interchangeably in software development, but they
serve different purposes and have distinct characteristics.

|                  | Library                                                                                                                                                                                                                    | Framework                                                                                                                                                                                                                                                                            |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Usage**        | A library is a collection of functions and procedures that a programmer can call in their application. The library provides specific functionality, but it's the developer's responsibility to explicitly call and use it. | A framework is a pre-built structure or scaffold that developers build their application within. It provides a foundation, enforcing a particular way of creating an application.                                                                                                    |
| **Control Flow** | In the case of a library, the control flow remains with the developer's application. The developer chooses when and where to use the library.                                                                              | With a framework, the control flow is inverted. The framework decides the flow of control by providing places for the developer to plug in their own logic (often referred to as "Inversion of Control" or IoC).                                                                     |
| **Nature**       | Libraries are passive in nature. They wait for the application's code to invoke their methods.                                                                                                                             | Frameworks are active and have a predefined flow of their own. The developer fills in specific pieces of the framework with their own code.                                                                                                                                          |
| **Example**      | Imagine you're building a house. A library would be like a toolbox with tools (functions) that you can use at will. You decide when and where to use each tool.                                                            | Using the house-building analogy, a framework would be like a prefabricated house where the main structure is already built. You're tasked with filling in the interiors and decor, but you have to follow the design and architecture already provided by the prefabricated design. |

## What is the difference between a `ratatui` (a library) and a [`tui-realm`](https://github.com/veeso/tui-realm/) (a framework)?

While `ratatui` provides tools (widgets) for building terminal UIs, it doesn't dictate or enforce a
specific way to structure your application. You need to decide how to best use the library in your
particular context, giving you more flexibility.

In contrast, `tui-realm` might provide more guidelines and enforcements about how your application
should be structured or how data flows through it. And, for the price of that freedom, you get more
features out of the box with `tui-realm` and potentially lesser code in your application to do the
same thing that you would with `ratatui`.

## What is the difference between `ratatui` and `cursive`?

[Cursive](https://github.com/gyscos/cursive) and Ratatui are both libraries that make TUIs easier to
write. Both libraries are great! Both also work on linux, macOS and windows.

### Cursive

Cursive uses a more declarative UI: the user defines the layout, then cursive handles the event
loop. Cursive also handles most input (including mouse clicks), and forwards events to the currently
focused view. User-code is more focused on "events" than on keyboard input. Cursive also supports
different backends like ncurses, pancurses, termion, and crossterm.

One of cursive's main features is its built-in event loop. You can easily attach callbacks to events
like clicks or key presses, making it straightforward to handle user interactions.

```rust
use cursive::views::{Dialog, TextView};

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Hello World!"))
                         .title("Cursive")
                         .button("Quit", |s| s.quit()));

    // Starts the event loop.
    siv.run();
}
```

![](https://user-images.githubusercontent.com/1813121/271896508-d5f6192c-d51b-4299-9b5e-9d91e4618f64.png)

### Ratatui

In Ratatui, the user handles the event loop, the application state, and re-draws the entire UI on
each iteration. It does not handle input and users have use another library (like
[crossterm](https://github.com/TimonPost/crossterm)). Ratatui supports Crossterm, termion, wezterm
as backends.

```rust
use ratatui::{prelude::*, widgets::*};

fn init() -> Result<(), Box<dyn std::error::Error>> {
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
  Ok(())
}

fn exit() -> Result<(), Box<dyn std::error::Error>> {
  crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  init()?;

  let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  loop {
    terminal.draw(|f| {
      let rect = centered_rect(f.size(), 35, 35);
      f.render_widget(
        Paragraph::new("Hello World!\n\n\n'q' to quit")
          .block(
            Block::default().title(block::Title::from("Ratatui").alignment(Alignment::Center)).borders(Borders::all()),
          )
          .alignment(Alignment::Center),
        rect,
      );
    })?;

    if crossterm::event::poll(std::time::Duration::from_millis(250))? {
      if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
        if key.code == crossterm::event::KeyCode::Char('q') {
          break;
        }
      }
    }
  }
  exit()?;

  Ok(())
}
```

![](https://user-images.githubusercontent.com/1813121/271896510-c8db19d1-f132-49b5-89da-c32cc21ab765.png)

You may have to write more code but you get precise control over exact UI you want to display with
Ratatui.

## Can you change font size in a terminal using `ratatui`?

`ratatui` itself doesn't control the terminal's font size. `ratatui` renders content based on the
size and capabilities of the terminal it's running in. If you want to change the font size, you'll
need to adjust the settings of your terminal emulator.

![](https://user-images.githubusercontent.com/1813121/269147939-0ed031f2-1977-4e92-b4b4-6c217d02e79b.png)

However, changing this setting in your terminal emulator will only change the font size for you
while you are developing your `ratatui` based application.

When a user zooms in and out using terminal shortcuts, that will typically change the font size in
their terminal. You typically will not know what the terminal font size is ahead of time.

However, you can know the current terminal size (i.e. columns and rows). Additionally, when zooming
in and out `ratatui` applications will see a terminal resize event that will contain the new columns
and rows. You should ensure your `ratatui` application can handle these changes gracefully.

You can detect changes in the terminal's size by listening for terminal resize events from the
backend of your choice and you can adjust your application layout as needed.

For example, here's how you might do it in
[crossterm](https://docs.rs/crossterm/0.27.0/crossterm/event/enum.Event.html#variant.Resize):

```rust
    match crossterm::terminal::read() {
        Ok(evt) => {
            match evt {
                crossterm::event::Event::Resize(x, y) => {
                    // handle resize event here
                },
                _ => {}
            }
        }
    }
```

:::tip

Since this can happen on the user end without your control, this means that you'll have to design
your `ratatui` based terminal user interface application to display content well in a number of
different terminal sizes.

:::

`ratatui` does support various styles, including bold, italic, underline, and more, and while this
doesn't change the font size, it does provide you with the capability to emphasize or de-emphasize
text content in your application.

Additionally you can use [`figlet`](https://docs.rs/figlet-rs/latest/figlet_rs/) or
[`tui-big-text`](https://github.com/joshka/tui-big-text/) to display text content across multiple
lines. Here's an example using [`tui-big-text`](https://github.com/joshka/tui-big-text/):

![[tui-big-text](https://github.com/joshka/tui-big-text/)](https://camo.githubusercontent.com/3a738ce21da3ae67660181538ef27473b86bebca73f42944e8012d52f86e500d/68747470733a2f2f7668732e636861726d2e73682f7668732d3364545474724c6b79553534684e61683232504152392e676966)

## Can you use multiple `terminal.draw()` calls consequently?

You _cannot_ use `terminal.draw()` multiple times in the same `main` loop.

Because Ratatui uses a double buffer rendering technique, writing code like this will **_NOT_**
render all three widgets:

```rust
  loop {
    terminal.draw(|f| {
      f.render_widget(widget1, f.size());
    })?;
    terminal.draw(|f| {
      f.render_widget(widget2, f.size());
    })?;
    terminal.draw(|f| {
      f.render_widget(widget3, f.size());
    })?;
    // handle events
    // manage state
  }
```

You want to write the code like this instead:

```rust
  loop {
    terminal.draw(|f| {
      f.render_widget(widget1, f.size());
      f.render_widget(widget2, f.size());
      f.render_widget(widget3, f.size());
    })?;
    // handle events
    // manage state
  }
```

## Should I use `stdout` or `stderr`?

When using `crossterm`, application developers have the option of rendering to `stdout` or `stderr`.

```rust
let mut t = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
// OR
let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
```

Both of these will work fine for normal purposes. The question you have to ask is how would you like
your application to behave in non-TTY environments.

For example, if you run `ratatui-application | grep foo` with `stdout`, your application won't
render anything to the screen and there would be no indication of anything going wrong. With
`stderr` the application will still render a TUI.

With `stdout`:

- Every app needs to add code to check if the output is a TTY and do something different based on
  the result
- App can't write a result to the user that can be passed in a pipeline, e.g.
  `my-select-some-value-app | grep foo`
- Tends to be what most command line applications do by default.

With `stderr`:

- No special setup necessary in order to run in a pipe command
- Unconventional and that might subvert users expectations
