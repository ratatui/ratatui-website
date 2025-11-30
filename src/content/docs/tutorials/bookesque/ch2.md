---
title: Chapter 2
tableOfContents: true
---

Welcome to Chapter 2 of this bookesque. Now that we've made our first TUI application, we'll now
learn techniques and best practices to make the app easy to work with and maintain.

## Using `struct`s to structure our code

After attempting Exercise 1 from the previous Chapter, you would've most likely come across
innumerable Ratatui applications that use an `App` or `Tui` struct to house the code and state for
the application. We'll be discussing that approach in this section.

### Dividing Our Application into Parts.

Analyzing our application from Chapter 1, we can notice 4 almost disjoint areas:

1. terminal setup,
2. rendering,
3. event handling, and
4. terminal teardown.

Each of these has code that can be encapsulated away from the rest of the application. Rendering and
event handling are tied together within our main event loop, which is the while loop that draws to
the terminal and handles events.

Now that we've identified a broad structure for the application, let's apply it to our current code.

### Applying the Structure to the Application

First, we'll define an `App` struct. This struct will have only one field, the `is_running` field.
It toggles the execution of our main application loop. According to our breakdown, we should have
four methods on this struct, namely `setup`, `render` (to render the application's widgets and UI),
`handle_events` (to handle key events) and `teardown`. Implementing this would look like the
following:

```rust
/// The main application which holds the state and logic of the application.
##[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    is_running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup() {
	    ratatui::init();
    }
    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

	pub fn teardown() {
		ratatui::restore();
	}
    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.is_running = false;
    }
}
```

We can identify all the parts from our previous application code here. There is a `render` function,
which creates and renders the same widgets that our code in the closure for `terminal.draw` did. The
`handle_crossterm_events` function filters out key events and passes them along to the
`on_key_event` function, which actually processed the keystrokes and performs the necessary actions.
Having these two functions separate (namely determining the type of event and handling it) allows us
to easily add support for mouse events later on, without heavy refactoring. The `quit`, `setup` and
`teardown` functions are pretty straightforward, and their functions are left as a (very simple)
exercise for the reader.

The keen eyed among you would've noticed that something is missing, and indeed something is missing:
a `run` function to run the application loop.

### Application Loop

In this very simple application, the event loop is trivial. However, as you start to build more
complex applications in Ratatui, the event loop becomes critical in determining how the rest of your
application is structured. For us though, it is pretty straightforward:

```rust
	pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.is_running = true;
        while self.is_running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }
```

The function starts the application (by setting `self.is_running` `true`). Then, in the `while`
loop, we call `terminal.draw` to render our UI, and then handle the crossterm events. When the loop
ends, we return.

### Polling events

Right now, reading events from the user blocks our application loop. That makes it impossible to do
stuff in-between key presses, and thus the speed of our application (the frame rate), is essentially
the number of keypresses per second. That isn't desirable for most applications, especially ones
that deal with real-time data. To solve this, we can use `Polling`. To do so, we first define a
timeout, which is how long our application will wait for a keypress.

```rust
const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(250);
```

Next, we wrap our `event::read()?` with the polling logic.

```rust
if event::poll(TIMEOUT)? {
	match event::read()? {
		// it's important to check KeyEventKind::Press to avoid handling key release events
		Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
		Event::Mouse(_) => {}
		Event::Resize(_, _) => {}
		_ => {}
	}
}
```

If the application doesn't receive a keystroke in `timeout` milliseconds (250 ms in our case), we
continue with our event loop.

## Single Source Model for TUI Application State.

In the end of Chapter 1, we had links to the `Singleton` design pattern. **Singleton** is a
creational design pattern, which ensures that only one object of its kind exists and provides a
single point of access to it for any other code.[^1] This allows multiple parts of our app (like
rendering and event handling), to have access to the same data. Although _true_ singletons are
pretty weird in Rust, the essence of one can be easily seen and used with a single `App` struct,
that is constructed once, and holds all the state of the application, so that all parts have access
to the same concurrent data. To give multiple parts access, we usually pass around `&mut`
references, which limits us to _linear_ event flow, meaning that rendering has to occur either
before or after the event handling. Due to Rust's borrowing rules (that there may only be one `&mut`
reference to an object at any given time; it is also written as "a mutable reference cannot be
aliased"), we can't simply create two mutable references to our `App` struct and pass them to two
concurrently running event loops. To combat this, a popular approach is to have a worker thread that
receives events, and then makes the changes in the state. This works via events and actions that are
sent via protocols like `mpsc` (Multiple Producer Single Consumer), which enables race-free
concurrent mutable access to state. For examples of this, check our the
[event-driven](https://github.com/ratatui/templates/tree/main/event-driven) and
[component](https://github.com/ratatui/templates/tree/main/component) templates.

## Exercises

1. Research about other architectures for TUI applications. Some popular ones include Component
   Architecture, Flux Architecture (introduced by Facebook) and Elm (used by frameworks like
   bubbletea in Golang).
2. Try coming up with a design pattern of your own. Figure out how to handle multiple types of
   events, their propagation, rendering loops, state management, and global data handling (like
   configurations, for example). All of these aspects will be covered in detail throughout this
   bookesque.

[^1]: https://refactoring.guru/design-patterns/singleton/rust/example
