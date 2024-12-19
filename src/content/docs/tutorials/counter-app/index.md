---
title: Counter App
sidebar:
  order: 0
---

The previous [Hello Ratatui] tutorial introduced how to create a simple TUI that displayed some text
and waited for the user to press a key. This tutorial will cover how to handle state and some more
complex interactions. You will build a counter application that allows the user to increment and
decrement a value on the screen.

When you're finished the application will look like the following:

![basic-app demo](./basic-app.png)

The application will render the counter in a [`Paragraph`] widget. When the user presses the left
and right arrow keys, the application will increment and decrement the value of the counter.

[Hello Ratatui]: /tutorials/hello-ratatui
[`Paragraph`]: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html
