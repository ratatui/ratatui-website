# Introduction

![](https://user-images.githubusercontent.com/24392180/244943746-93ab0e38-93e0-4ae0-a31b-91ae6c393185.gif)

`ratatui` is based on the principle of immediate rendering with intermediate
buffers. This means that at each new frame you have to build all widgets that
are supposed to be part of the UI. In short, the `ratatui` library handles
drawing to the terminal.

It is important to note that `ratatui`:

- does **NOT** handle keyboard input events
- does **NOT** modify the state of your application
- does **NOT** dicate how you want to structure your application

The `ratatui` crate is a library and not a framework. And the `ratatui` library
provides widgets that allows a developer to imperatively declare what the view
of your application should look like, and then allows a developer to draw those
widgets efficiently to the terminal.

For these reasons, `ratatui` can be highly flexible and customizable. And while
this can be empowering, it also does put the onus on developers using `ratatui`
to understand how to best architect their applications, to tailor the experience
for users as they see fit.

## Who is ratatui for?

`ratatui` is designed for developers and enthusiasts who:

- appreciate the retro aesthetic of the terminal,
- want a lightweight alternative to graphical user interfaces (GUIs),
- need applications that are to be deployed in constrained environments, like on
  servers with limited resources, and
- prefer to have full control over input and events, allowing for a more
  customized and tailored user experience.

## Who is this book for?

In this book, we will cover beginner guides to advanced patterns for developing
terminal user interfaces.

Those new to the world of TUIs will find this book a comprehensive guide,
introducing the foundational concepts and walking through common patterns of
using `ratatui`. Additionally, developers who have worked with TUIs will
understand the nuances and benefits of using `ratatui`.

We hope that this book can be a journey into creating beautiful and functional
terminal-based applications.
