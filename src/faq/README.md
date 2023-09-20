# FAQ

- [Duplicate Key Events on Windows](#why-am-i-getting-duplicate-key-events-on-windows)
- [`tokio` / `async`](#when-should-i-use-tokio-and-asyncawait)
- [`tui.rs` history](#what-is-the-relational-between-tui-rs-and-ratatui)

## Why am I getting duplicate key events on Windows?

{{#include ./duplicate-key-events-windows.md:4:}}

## When should I use `tokio` and `async`/`await`?

{{#include ./tokio-async.md:2:}}

## What is the relational between `tui-rs` and `ratatui`?

{{#include ./tui-rs-history.md:2:}}

## What is the difference between a library and a framework?

The terms library and framework are often used interchangeably in software development, but they
serve different purposes and have distinct characteristics.

|                  | Library                                                                                                                                                                                                                    | Framework                                                                                                                                                                                                                                                                            |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Usage**        | A library is a collection of functions and procedures that a programmer can call in their application. The library provides specific functionality, but it's the developer's responsibility to explicitly call and use it. | A framework is a pre-built structure or scaffold that developers build their application within. It provides a foundation, enforcing a particular way of creating an application.                                                                                                    |
| **Control Flow** | In the case of a library, the control flow remains with the developer's application. The developer chooses when and where to use the library.                                                                              | With a framework, the control flow is inverted. The framework decides the flow of control by providing places for the developer to plug in their own logic (often referred to as "Inversion of Control" or IoC).                                                                     |
| **Nature**       | Libraries are passive in nature. They wait for the application's code to invoke their methods.                                                                                                                             | Frameworks are active and have a predefined flow of their own. The developer fills in specific pieces of the framework with their own code.                                                                                                                                          |
| **Example**      | Imagine you're building a house. A library would be like a toolbox with tools (functions) that you can use at will. You decide when and where to use each tool.                                                            | Using the house-building analogy, a framework would be like a prefabricated house where the main structure is already built. You're tasked with filling in the interiors and decor, but you have to follow the design and architecture already provided by the prefabricated design. |

## What is the difference between a `ratatui` (a library) and a `tui-realm` (a framework)?

While `ratatui` provides tools (widgets) for building terminal UIs, it doesn't dictate or enforce a
specific way to structure your application. You need to decide how to best use the library in your
particular context, giving you more flexibility.

In contrast, `tui-realm` might provide more guidelines and enforcements about how your application
should be structured or how data flows through it. And, for the price of that freedom, you get more
features out of the box with `tui-realm` and potentially lesser code in your application to do the
same thing that you would with `ratatui`.

## Can you change font size in a terminal using `ratatui`?

`ratatui` itself doesn't control the terminal's font size. `ratatui` renders content based on the
size and capabilities of the terminal it's running in. If you want to change the font size, you'll
need to adjust the settings of your terminal emulator.

![](https://user-images.githubusercontent.com/1813121/269147939-0ed031f2-1977-4e92-b4b4-6c217d02e79b.png)

However, changing this setting in your terminal emulator will only change the font size for you
while you are developing your `ratatui` based application.

When a user zooms in and out using terminal shortcuts, that will typically change the font size in
their terminal. You typically will not know what the terminal font size is ahead of time.

However, you can know the current terminal size (i.e. columns and rows). Additionally, when zooming
in and out `ratatui` applications will see a terminal resize event that will contain the new columns
and rows. You should ensure your `ratatui` application can handle these changes gracefully.

You can detect changes in the terminal's size by listening for terminal resize events from the
backend of your choice and you can adjust your application layout as needed.

For example, here's how you might do it in
[crossterm](https://docs.rs/crossterm/0.27.0/crossterm/event/enum.Event.html#variant.Resize):

```rust
    match crossterm::terminal::read() {
        Ok(evt) => {
            match evt {
                crossterm::event::Event::Resize(x, y) => {
                    // handle resize event here
                },
                _ => {}
            }
        }
    }
```

```admonish tip
Since this can happen on the user end without your control, this means that you'll have to design
your `ratatui` based terminal user interface application to display content well in a
number of different terminal sizes.
```

`ratatui` does support various styles, including bold, italic, underline, and more, and while this
doesn't change the font size, it does provide you with the capability to emphasize or de-emphasize
text content in your application.

Additionally you can use [`figlet`](https://docs.rs/figlet-rs/latest/figlet_rs/) or
[`tui-big-text`](https://github.com/joshka/tui-big-text/) to display text content across multiple
lines. Here's an example using [`tui-big-text`](https://github.com/joshka/tui-big-text/):

![[tui-big-text](https://github.com/joshka/tui-big-text/)](https://camo.githubusercontent.com/3a738ce21da3ae67660181538ef27473b86bebca73f42944e8012d52f86e500d/68747470733a2f2f7668732e636861726d2e73682f7668732d3364545474724c6b79553534684e61683232504152392e676966)
