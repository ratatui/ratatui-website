# Introduction

`ratatui` is based on the principle of immediate rendering with intermediate buffers.
This means that at each new frame you have to build all widgets that are supposed to be part of the UI.
In short, the `ratatui` library handles drawing to the terminal.

![](https://user-images.githubusercontent.com/24392180/244943746-93ab0e38-93e0-4ae0-a31b-91ae6c393185.gif)

`ratatui` does not provide any input handling nor any event system out of the box.
Getting keyboard input events, modifying the state of your application based on those events and figuring out which widgets best reflect the view of the state of your application are the reasons that `ratatui` is highly flexible and customizable.
While this can be empowering, it does put onus on developers using `ratatui` to understand how to best architect their applications to tailor the experience for users as they see fit.

## Who is ratatui for?

`ratatui` is designed for developers and enthusiasts who:

- appreciate the retro aesthetic of the terminal,
- want a lightweight alternative to graphical user interfaces (GUIs),
- need applications that are to be deployed in constrained environments, like on servers with limited resources, and
- prefer to have full control over input and events, allowing for a more customized and tailored user experience.

## Who is this book for?

In this book, we will cover beginner guides to advanced patterns for developing terminal user
interfaces.

Those new to the world of TUIs will find this book a comprehensive guide, introducing the foundational concepts and walking through common patterns of using `ratatui`.
Additionally, developers who have worked with TUIs will understand the nuances and benefits of using `ratatui`.

We hope that this book can be a journey into creating beautiful and functional terminal-based applications.
