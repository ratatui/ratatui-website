# Introduction

![Demo](https://raw.githubusercontent.com/ratatui-org/ratatui/images/examples/demo2.gif)

## What is ratatui?

`ratatui` is a [Rust](https://www.rust-lang.org/) crate that provides widgets allowing you to
imperatively declare what the view of your application should be, and then draws those widgets
efficiently to the terminal.

`ratatui` is based on the principle of immediate rendering. This means that at each new frame all
widgets that are supposed to be part of the UI are re-built.

**The `ratatui` crate is a library and not a framework.**

It is important because `ratatui`:

- does _not_ handle keyboard input events
- does _not_ modify the state of your application
- does _not_ dicate how you want to structure your application

`ratatui` can be highly flexible and customizable. And while this can be empowering, it also puts
the onus on developers using `ratatui` to understand how to best architect their applications, to
tailor the experience for users as they see fit.

## Who is ratatui for?

`ratatui` is designed for developers and enthusiasts who:

- appreciate the retro aesthetic of the terminal,
- want a lightweight alternative to graphical user interfaces (GUIs),
- need applications that are to be deployed in constrained environments, like on servers with
  limited resources, and
- prefer to have full control over input and events, allowing for a more customized and tailored
  user experience.

## Who is this book for?

In this book, we will cover beginner guides to advanced patterns for developing terminal user
interfaces.

Those new to the world of TUIs will find this book a comprehensive guide, introducing the
foundational concepts and walking through common patterns of using `ratatui`. Additionally,
developers who have worked with TUIs will understand the nuances and benefits of using `ratatui`.

We hope that this book can be a journey into creating beautiful and functional terminal-based
applications.

```admonish note
We want to hear your feedback and suggestions.

Feel free to give some suggestions on improving the book or documentation via
[GitHub Discussions](https://github.com/ratatui-org/ratatui-book/discussions) or chat with us on
`#doc-discussion` on [Discord](https://discord.gg/pMCEU9hNEj).
```
