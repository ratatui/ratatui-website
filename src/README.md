# Introduction to Ratatui

![Demo](https://raw.githubusercontent.com/ratatui-org/ratatui/images/examples/demo2.gif)

[Ratatui] is a Rust library for cooking up delicious text user interfaces (TUIs). It is a
lightweight library that provides a set of widgets and utilities to build simple or complex rust
TUIs.

Ratatui is an [immediate mode] graphics library. Applications imperatively declare how to render
each frame in full by combining widgets and layout. Ratatui then draws the described UI widgets
efficiently to the terminal.

Applications built with Ratatui use the features of their chosen [backend] ([Crossterm], [Termion],
or [Termwiz] to handle:

- keyboard input events
- mouse events
- switching to raw mode and the alternate screen

Ratatui is very flexible and customizable. It does not dictate how you need to structure your
application, as it is a library not a framework.
This book covers some different options covering the range from simple single file applications
throught more complex applications using approaches based on components, Flux and The Elm
Architecture.

## Who is ratatui for?

Ratatui is designed for developers and enthusiasts who:

- want a lightweight alternative to graphical user interfaces (GUIs),
- need applications that are to be deployed in constrained environments, like on servers with
  limited resources, and
- prefer to have full control over input and events, allowing for a more customized and tailored
  user experience.
- appreciate the retro aesthetic of the terminal,

## Who is this book for?

In this book, we will cover beginner guides to advanced patterns for developing terminal user
interfaces.

Those new to the world of TUIs will find this book a comprehensive guide, introducing the
foundational concepts and walking through common patterns of using Ratatui. Additionally, developers
who have worked with TUIs will understand the nuances and benefits of using Ratatui.

We hope that this book can be a journey into creating beautiful and functional terminal-based
applications.

[immediate mode]: <https://en.wikipedia.org/wiki/Immediate_mode_(computer_graphics)>
[backend]: ./concepts/backends
[Ratatui]: <https://crates.io/crates/ratatui>
[Crossterm]: <https://crates.io/crates/crossterm>
[Termion]: <https://crates.io/crates/termion>
[Termwiz]: <https://crates.io/crates/termwiz>

```admonish note
Help Us Improve!

We've designed this user guide to aid you throughout your journey with our open-source project.
However, the beauty of open source is that it's not just about receiving, but also contributing. We
highly encourage you to contribute to our project and help improve it even further. If you have
innovative ideas, helpful feedback, or useful suggestions, please don't hesitate to share them with us.

If you see something that could be better written, feel free to [create an issue], a
[discussion thread] or even contribute a [Pull Request]. We're also often active in the
`#doc-discussion` channel on [Discord] and [Matrix]

[create an issue]: https://github.com/ratatui-org/ratatui-book/issues
[discussion thread]: https://github.com/ratatui-org/ratatui-book/discussions
[Pull Request]: https://github.com/ratatui-org/ratatui-book/pulls
[Discord]: https://discord.gg/pMCEU9hNEj
[Matrix]: https://matrix.to/#/#ratatui:matrix.org
```
