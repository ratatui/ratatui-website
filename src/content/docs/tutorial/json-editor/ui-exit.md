---
title: The Exit Popup
---

We have a way for the user to view their already entered key-value pairs, and we have a way for the
user to enter new ones. The last screen we need to create, is the exit/confirmation screen.

In this screen, we are asking the user if they want to output the key-value pairs they have entered
in the `stdout` pipe, or close without outputting anything.

```rust
{{#include @code/ratatui-json-editor-app/src/ui.rs:exit_screen}}
```

The only thing in this part that we haven't done before, is use the
[`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html) widget. This is a
special widget that does what the name suggests --- it clears everything in the space it is
rendered.
