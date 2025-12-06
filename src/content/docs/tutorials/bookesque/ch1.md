---
title: Chapter 1
tableOfContents: true
---

Welcome the world of Ratatui! This chapter will teach you what Ratatui is, and the core fundamentals
to building your first TUI. In essence, there are three broad parts of a Ratatui application:

1. UI rendering loop (rendering your TUI to the screen)
2. Event Management (Keypresses, mouse click, etc.)
3. State Management (Making your app dynamic)

For all these options, Ratatui is incredibly flexible, allowing you to employ multiple approaches to
tackle these parts. But let's not get ahead of ourselves. The rat hole of architectures and
approaches is deep, and we'll explore it later on in this series, but to do that, we need to know
how a Ratatui TUI runs.

```svgbob
          +------------+
          | App starts |
          +------+-----+
                 |
        +--------v---------+
        |Initial UI Renders|
        +--------+---------+
                 |
 +---------------v---------------+
 |       +----------------+      |
 |       | Event Received |<--+  |
 |       +-------+--------+   |  |
 |               |            |  |
 |               |            |  |
 |      +--------v---------+  |  |
++-quit-+App Handles Events|  |  |
||      +--------+---------+  |  |
||               |            |  |
||               |            |  |
||        +------v-------+    |  |
||        |Renders output+----+  |
||        +--------------+       |
|+-------------------------------+
|
+----------------+
                 |
       +---------v----------+
       |App Cleanup and Exit|
       +--------------------+
```

## App Flow

The application flow can be separated into a startup (done using `ratatui::init()`), an application
loop consisting of event handling and rendering, and a cleanup to restore the user's terminal back
to its original state (done using `ratatui::restore`). The startup and cleanup are relatively
simple, so we'll start with those first.

### Startup and Cleanup

#### Startup

Setting up the terminal and necessary panic hooks for the application (we'll talk about why we need
to later) can be done with a simple one liner:

```rust
let terminal = ratatui::init();
```

Under the hood, the `init()` function does the following:

1. Sets the panic hook. A [_panic hook_](http://localhost:4321/recipes/apps/panic-hooks/) is a
   custom function that is run automatically when a thread panics, before the default panic handler
   takes over. In Ratatui, we use it to restore the user's terminal incase the application panics,
   so the user's terminal isn't left unusable due to our application crashing.
   ```rust
    fn set_panic_hook() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(alloc::boxed::Box::new(move |info| {
            restore();
            hook(info);
        }));
    }
   ```
2. Enables _raw mode_ in the terminal. In a terminal, raw mode is a configuration where the
   operating system's terminal driver disables all special processing of input and output, passing
   data directly to the application as it is typed. This allows for Ratatui to precisely render
   content to the terminal, without interference, and for proper event capture.
3. Enters the [_Alternate Screen_](https://ratatui.rs/concepts/backends/alternate-screen/). The
   alternate screen is a separate buffer that some terminals provide, distinct from the main screen.
   When activated, the terminal will display the alternate screen, hiding the current content of the
   main screen. Applications can write to this screen as if it were the regular terminal display,
   but when the application exits, the terminal will switch back to the main screen, and the
   contents of the alternate screen will be cleared. See
   [this page](https://ratatui.rs/concepts/backends/alternate-screen/##_top) on Alternate screens
   for more information. And that's it for the startup!

#### Cleanup

The cleanup does the exact opposite of what the startup `init()` does. With

```rust
ratatui::restore();
```

we disable raw mode, and leave the alternate screen, which gives the user their terminal back. This
is the same function that is called by our panic hook, to restore the terminal to its original
state.

### Rendering and event handling

:::note

To make life much easier, we'll use `color-eyre` for a general error type, so that we can use error
propagation to reduce boilerplate. Simply run:

```bash
cargo add color-eyre
```

to get started.

:::

### Rendering

This is where we get spoilt for choice. There are many ways in which we can do this, ranging from
sub-50 lines of code to file-spanning multithreaded events loops with fancy stuff like _Refined
method calls_, but for now, we'll start slow.

This doesn't have to be complicated. To start, we'll create a simple loop within our main function.

```rust
fn main() -> color_eyre::Result<()> {
    // -- snip --
    loop {
        // event handling and rendering
        // here
    }
    Ok(())
}
```

Perfect! Now, let's populate the loop. But before that, we need a way to know if our app is running,
so right before the loop, we'll create a variable to track that.

```rust
fn main() -> color_eyre::Result<()> {
    // -- snip --
    let mut is_running = true;
    while is_running {
        // event handling and rendering
        // here
    }
    Ok(())
}
```

Now that we have that done, let's get to the rendering. For that, we'll use the `draw` method on the
`DefaultTerminal` struct returned by `ratatui::init()`. It accepts a `FnOnce(&mut Frame)`, meaning
that it takes a function or a closure that accepts a `Frame` argument. The `Frame` is a consistent
view into the terminal state for rendering a single frame. It is used to render widgets to the
terminal and control the cursor position. From this `Frame`, we can get the `area` in which we'll
render our application. That would look like the following.

```rust
while is_running {
	terminal.draw(|frame| {
		let area = frame.area();
	})?;
}
```

Great! Now we have a `Frame`, and area to render to. But we don't have anything to render. That's
where `Widget`s come in.

:::note

#### Widgets

This is a short intro to widgets. For that check out the dedicated
[page on Widgets](https://ratatui.rs/concepts/widgets/)

Widgets are reusable components, which are really structs which have the necessary state to operate,
and implement either the `Widget` or `StatefulWidget` traits, depending on if the widget has
persistent state that it needs to work. Ratatui includes numerous built-in widgets, of which we'll
use the `Line` and `Paragraph` widgets.

:::

Now that we know (in very brief), what widgets are, we can start using them.

```rust
terminal.draw(|frame| {
	let area = frame.area();
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
})?;
```

That was a lot of code that we just saw. Let's walk through it and see what it does. First, create a
new `Line` widget, which is used to display a single line of text. Using methods from the `Stylize`
trait (provided by ratatui), we color it blue, and make it bold. Then we define the text that we
want our `Paragraph` to show. Then, we render the `Paragraph` widget to the frame using its `render`
method. We create a new `Paragraph`, which is a widget that displays a paragraph of text, which has
a bordered `Block`. A `Block` is a very useful widget that allows us to add borders and padding
around widgets. We then use the `Line` we constructed previously to be the title for this block.

Phew! That was a lot of information. Let's get to event handling now.

### Event handling

To handle events, we use crossterm to read the current events, and then we can match them to see if
that key does anything. To do that, we'll create a new function, called `on_key_event`.

```rust
fn on_key_event(key: KeyEvent, is_running: &mut bool) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
            *is_running = false;
        }
        _ => {}
    }
}
```

Matching against the `KeyEvent`'s modifier and key code, we stop the app if the user presses either
`q`, `Esc` or `CONTROL-c`. To read key events, we use `crossterm::event::read`. We filter out the
`KeyEvents`, and importantly, check if the key was a key press. This is done because Windows sends
two events when a key is pressed, one when it is pressed, and another for its release. By filtering
our the latter, we ensure that keystrokes are registered twice.

```rust
match event::read()? {
	// it's important to check KeyEventKind::Press to avoid handling key release events
	Event::Key(key) if key.kind == KeyEventKind::Press => {
		on_key_event(key, &mut is_running)
	}
	Event::Mouse(_) => {}
	Event::Resize(_, _) => {}
	_ => {}
};
```

And, just like that, you've created your first Ratatui application.

## Challenge for the Reader

1. Think of a way to make the code modular. The code right now is hard to refactor, as everything
   has been shoved into the same loop. Find a way (or multiple), to make the code modular, and make
   it easier to work with. (Note: Think of `is_running` as global state)
2. What will happen if the app needs to do things in-between events? Right now, the event reading
   blocks our main thread, which means that we can't do anything until an event occurs. (Note: Read
   about `Polling` in crossterm).

## Further Reading:

1. https://ratatui.rs/concepts/widgets/
2. https://ratatui.rs/concepts/event-handling/
3. https://ratatui.rs/recipes/apps/panic-hooks/
4. https://ratatui.rs/recipes/apps/better-panic/
5. https://ratatui.rs/recipes/apps/color-eyre/
6. https://refactoring.guru/design-patterns/singleton/rust/example and
   https://en.wikipedia.org/wiki/Singleton_pattern A (pseudo) Singleton pattern is the predominant
   way for storing state in Ratatui. I call it a pseudo-Singleton pattern because a _true_ singleton
   is only possible with unsafe (see the first link).
