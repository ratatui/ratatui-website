# Rendering

The world of UI development consists mainly of two dominant paradigms: retained mode and immediate
mode. Most traditional GUI libraries operate under the retained mode paradigm. However, `ratatui`
employs the immediate mode rendering approach. for TUI development.

This makes `ratatui` is different from GUI frameworks you might use, because it only updates when
you tell it to.

## What is Immediate Mode Rendering?

Immediate mode rendering is a UI paradigm where the UI is recreated every frame. Instead of creating
a fixed set of UI widgets and updating their state, you "draw" your UI from scratch in every frame
based on the current application state.

In a nutshell:

- Retained Mode: You set up your UI once, create widgets, and later modify their properties or
  handle their events.
- Immediate Mode: You redraw your UI every frame based on your application state. There's no
  permanent widget object in memory.

In `ratatui`, every frame draws the UI anew.

```rust
loop {
    terminal.draw(|f| {
        if state.condition {
            f.render_widget(SomeWidget::new(), layout);
        } else {
            f.render_widget(AnotherWidget::new(), layout);
        }
    })?;
}
```

[This article](https://caseymuratori.com/blog_0001) and the accompanying YouTube video is worth your
time if you are new to the immediate mode rendering paradigm.

<iframe width="560" height="315" src="https://www.youtube.com/embed/Z1qyvQsjK5Y?si=eiBHXiXIo3Z0u2zs" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

This 4 minute talk about `IMGUI` is also tangentially relevant.

<iframe width="560" height="315" src="https://www.youtube.com/embed/LSRJ1jZq90k?si=8NB5yiZ8IGS_QE_E" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## Advantages of Immediate Mode Rendering

- **Simplicity**: Without a persistent widget state, your UI logic becomes a direct reflection of
  your application state. You don't have to sync them or worry about past widget states.
- **Flexibility**: You can change your UI layout or logic any time, as nothing is set in stone. Want
  to hide a widget conditionally? Just don't draw it based on some condition.

## Disadvantages of Immediate Mode Rendering

It is important to understand that only calling `Backend.draw()` will actually output anything to
the screen. What it means is _you_, the programmer, are responsible for keeping the TUI responsive.
If you accidentally block the thread that updates the UI, it will not update until the thread
unblocks.

`ratatui` in particular only handles how widget would be rendered. If you have to use a supported
third party library, e.g. `crossterm` to actually draw to the terminal. In addition, you also have
to use `crossterm` to read key inputs.

Out of the box, there's little to no help in organizing large applications. The onus is on the
developer using `ratatui` to be principled (or not).
