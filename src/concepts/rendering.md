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

<iframe width="560" height="315" src="https://www.youtube.com/embed/Z1qyvQsjK5Y?si=eiBHXiXIo3Z0u2zs"
 title="YouTube video player" frameborder="0"
 allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
 allowfullscreen></iframe>

This 4 minute talk about `IMGUI` is also tangentially relevant.

<iframe width="560" height="315" src="https://www.youtube.com/embed/LSRJ1jZq90k?si=8NB5yiZ8IGS_QE_E"
 title="YouTube video player" frameborder="0"
 allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
 allowfullscreen></iframe>

### Advantages of Immediate Mode Rendering

- **Simplicity**: Without a persistent widget state, your UI logic becomes a direct reflection of
  your application state. You don't have to sync them or worry about past widget states.
- **Flexibility**: You can change your UI layout or logic any time, as nothing is set in stone. Want
  to hide a widget conditionally? Just don't draw it based on some condition.

### Disadvantages of Immediate Mode Rendering

- **Render loop management**: In Immediate mode rendering, the onus of rendering lies solely on the
  programmer. Every visual update necessitates a call to `Backend.draw()`. Hence, if the rendering
  thread is inadvertently blocked, the UI will not update until the thread resumes.

  ```admonish note
  The `ratatui` library in particular only handles how widget would be rendered to a "Backend", e.g.
  `crossterm`. The `Backend` in question would use an external crate e.g. `crossterm` for actually
  drawing to the terminal.
  ```

- **Event loop orchestration**: Along with managing "the render loop", developers are also
  responsible for handling "the event loop". This involves deciding on a third-party library for the
  job. `crossterm` is a popular crate to handle key inputs and you'll find plenty of examples in the
  repository and online for how to use it. `crossterm` also supports a `async` event stream, if you
  are interested in using `tokio`.

- **Architecture design considerations**: With `ratatui`, out of the box, there's little to no help
  in organizing large applications. Ultimately, the decision on structure and discipline rests with
  the developer to be principled.
