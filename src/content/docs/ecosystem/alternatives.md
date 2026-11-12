---
title: Alternatives
description: Other Rust libraries for building terminal user interfaces, and how they compare.
sidebar:
  order: 6
---

Ratatui is not the only way to build a terminal interface in Rust. This page covers other libraries
and where each one is a good fit.

## How Ratatui compares

Ratatui is an immediate-mode library: every frame, your app renders its widgets into a buffer, and
Ratatui diffs that buffer against the last one and writes the changes to the terminal. Your app owns
the event loop, state, and input handling. Widgets don't have to be rebuilt every frame, though.
Many apps keep long-lived widgets that hold their own state and
[render them by reference](/concepts/widgets/).

Ratatui is also the most widely used of these libraries, which means there's a lot built on it:

- [Built-in widgets](/showcase/widgets/) for tables, lists, charts, sparklines, gauges, canvases,
  and more.
- [Third-party widgets](/showcase/third-party-widgets/) and
  [Awesome Ratatui](https://github.com/ratatui/awesome-ratatui) for text areas, tree views, menus,
  embedded terminals, images, and more.
- Backends for crossterm, termion, and termwiz, plus ways to run Ratatui
  [in the browser](/ecosystem/ratzilla/), on [embedded displays](/ecosystem/mousefood/), and on
  [UEFI](/ecosystem/uefi/).
- Many [apps](/showcase/apps/), templates, and tutorials to learn from.

| Library               | Model                             | Event loop | Built-in controls                          | Best for                                  | Repos (GitHub) | Crates (direct) | Status (Nov 2026)               |
| --------------------- | --------------------------------- | ---------- | ------------------------------------------ | ----------------------------------------- | -------------- | --------------- | ------------------------------- |
| [Ratatui](/)          | Immediate mode                    | Yours      | Widgets, no input/focus                    | Dashboards, custom apps, full control     | 20,200 repos   | 6,900           | Active                          |
| [AppCUI](#appcui)     | Retained or immediate             | AppCUI's   | Many, with themes                          | Desktop-style apps with windows and menus | 3 repos        | 2               | Active                          |
| [Cursive](#cursive)   | Retained views, callbacks         | Cursive's  | Many, with focus and themes                | Form and dialog style apps                | 4,300 repos    | 161             | Active, last release 2024       |
| [iocraft](#iocraft)   | Declarative components and hooks  | iocraft's  | Input, button, checkbox, scroll view       | CLI output and React-style TUIs           | 96 repos       | 15              | Active                          |
| [R3BL TUI](#r3bl-tui) | Elm/React-inspired, async         | R3BL's     | Markdown editor, dialogs, readline, picker | Rich async apps with an editor component  | 52 repos       | 6               | Active                          |
| [RxTUI](#rxtui)       | Retained components with messages | RxTUI's    | Text input, spinner                        | Experimenting with a DOM-style model      | None listed    | 0               | **Archived**                    |
| [termwiz](#termwiz)   | Terminal abstraction and surfaces | Yours      | Minimal                                    | Terminal emulator-grade control           | 6,800 repos\*  | 49              | Low activity, last release 2025 |
| [uncurses](#uncurses) | Cell grid with a diffing renderer | Yours      | None                                       | Low-level rendering on modern terminals   | 4 repos        | 2               | Active, new in 2026             |

"Repos" is the number of GitHub repositories that GitHub's dependency graph lists as depending on
each project's main crate. It counts every crate in a repository's lockfile, so it includes indirect
use. "Crates" is the number of crates on crates.io that depend on it directly. Both are as of
November 2026.

\*Almost all of termwiz's repository count comes from Ratatui's termwiz backend. GitHub only lists
the 1,200 most recent dependents. In a sample of 110 of those with a lockfile, 106 got termwiz
through `ratatui-termwiz`, 2 through another Ratatui-based crate, and 2 used it directly.

## AppCUI

[AppCUI-rs](https://github.com/gdt050579/AppCUI-rs) is a cross-platform framework for building
desktop-style applications in the terminal. It offers both retained-mode and immediate-mode APIs, a
layout system, and a broad set of controls: windows, menus, list and tree views, date and color
pickers, and bar, line, and pie charts. Its frame-based mode also supports timer-driven screens that
don't use windows at all.

**Strengths:**

- The widest range of built-in controls of the libraries here, closer to a GUI toolkit than a
  rendering library.
- Windows, menus, and themes come ready to use, which suits tools that resemble classic Turbo Vision
  or Norton Commander style apps.
- A book and screenshot gallery document the controls.

![AppCUI-rs basic controls demo](https://gdt050579.github.io/AppCUI-rs/gallery/img/basic_controls/basic_controls.gif)

- [GitHub](https://github.com/gdt050579/AppCUI-rs)
- [Crate](https://crates.io/crates/appcui)
- [Documentation](https://docs.rs/appcui)
- [Book](https://gdt050579.github.io/AppCUI-rs/)
- [Gallery](https://gdt050579.github.io/AppCUI-rs/gallery/screenshots.html)

## Cursive

[Cursive](https://github.com/gyscos/cursive) is one of the oldest and most widely used Rust TUI
libraries. You build a tree of views (dialogs, text views, lists, edit fields, menus) and attach
callbacks to them. Cursive runs the event loop, routes input, and handles focus between views,
though you can also step the loop yourself with `Cursive::runner`. Other threads can send updates
into the UI through `cb_sink`. It uses the crossterm backend by default, with others available.

GitHub lists about 4,300 repositories that depend on it, including the Spotify client
[ncspot](https://github.com/hrkfdn/ncspot) and Meta's
[below](https://github.com/facebookincubator/below) system monitor.

**Strengths:**

- A large set of interactive views out of the box, including text input, select lists, menubars, and
  layered dialogs.
- Focus, keyboard navigation, and mouse handling are built in, so form-heavy apps need little code.
- Mature, with tutorials and a range of third-party views. Development is ongoing, though the last
  crates.io release (0.21.1) was in August 2024.

Cursive's maintainer has written a
[comparison with Ratatui](https://github.com/gyscos/cursive/wiki/Cursive-vs-ratatui).

![A Cursive dialog example](https://raw.githubusercontent.com/gyscos/cursive/main/doc/cursive_example.png)

- [GitHub](https://github.com/gyscos/cursive)
- [Crate](https://crates.io/crates/cursive)
- [Documentation](https://docs.rs/cursive)

## iocraft

[iocraft](https://github.com/ccbrown/iocraft) is a declarative library modeled on React. You write
components with an `element!` macro, manage state with hooks, and lay out content with flexbox
(powered by [taffy](https://docs.rs/taffy/)). It can print styled output once, like a rich
`println!`, or run an interactive full-screen app.

**Strengths:**

- Familiar if you know React: components, props, context, and hooks.
- Flexbox layout instead of constraint-based splitting.
- Handles both one-shot styled CLI output and interactive interfaces, with async support.

![A Hello World interface rendered with iocraft](https://raw.githubusercontent.com/ccbrown/iocraft/refs/heads/main/examples/images/hello-world.png)

- [GitHub](https://github.com/ccbrown/iocraft)
- [Crate](https://crates.io/crates/iocraft)
- [Documentation](https://docs.rs/iocraft)

## R3BL TUI

[R3BL TUI](https://github.com/r3bl-org/r3bl-open-core/tree/main/tui) is an async framework inspired
by React and Elm. Apps are built from components registered with the app, with state updated through
async message passing, flexbox-like layout, and CSS-like styling. It has its own rendering engine
and, on Linux, its own terminal I/O layer; other platforms use crossterm. It is developed by
[R3BL](https://r3bl.com), the company of former Google engineer
[Nazmul Idris](https://developerlife.com/about-me/).

**Strengths:**

- Async from the ground up, built on Tokio.
- Includes heavier pieces such as a Markdown editor, modal dialogs with async autocomplete, an async
  readline, and an inline `choose()` picker. The readline and picker work on their own, without the
  full-screen framework.
- Styling with a CSS-like stylesheet model.

- [GitHub](https://github.com/r3bl-org/r3bl-open-core)
- [Crate](https://crates.io/crates/r3bl_tui)
- [Documentation](https://docs.rs/r3bl_tui)

## RxTUI

[RxTUI](https://github.com/superradcompany/rxtui) uses a retained-mode, DOM-style component model.
Components hold their own state and update through messages, combining Elm's message architecture
with React-style composition. It handles diffing and dirty tracking for you and supports inline
rendering for CLI tools.

**Strengths:**

- Declarative components with message-based updates, so you don't write a render loop.
- Automatic diffing and minimal redraws.
- Inline mode for output that stays in the terminal history.

**Keep in mind** that the repository is archived. Its last release was 0.1.8 in December 2025, when
it still described itself as early in development, so expect no further fixes.

- [GitHub](https://github.com/superradcompany/rxtui)
- [Crate](https://crates.io/crates/rxtui)
- [Documentation](https://docs.rs/rxtui)

## termwiz

[termwiz](https://github.com/wezterm/wezterm/tree/main/termwiz) is the terminal library behind the
[WezTerm](https://wezterm.org) terminal emulator. It provides terminal capability handling, input
parsing, and _surfaces_ that track changes and compute minimal updates. It also includes a basic
widget layer. It was written by [Wez Furlong](https://wezfurlong.org), the author of WezTerm.

**Strengths:**

- Built and tested as part of a full terminal emulator.
- Detailed escape sequence parsing and capability handling.
- Surfaces with change tracking, suited to building your own rendering layer.

termwiz is also one of Ratatui's [backends](/concepts/backends/), so you can use it under Ratatui
instead of choosing between them. It is developed inside the WezTerm repository; the last crates.io
release (0.23.3) was in March 2025.

- [GitHub](https://github.com/wezterm/wezterm/tree/main/termwiz)
- [Crate](https://crates.io/crates/termwiz)
- [Documentation](https://docs.rs/termwiz)

## uncurses

[uncurses](https://uncurses.org) is a low-level terminal rendering library by
[Ayman Bagabas](https://github.com/aymanbagabas), formerly at
[Charm](https://github.com/charmbracelet), where they were the most active contributor to
[Bubble Tea](https://github.com/charmbracelet/bubbletea) and the main author of its rendering
library, [Ultraviolet](https://github.com/charmbracelet/ultraviolet). It gives you a cell grid with
a diffing renderer and typed input events, and leaves the event loop and application structure to
you. Rather than reading terminfo, it queries the terminal for the features it supports, and uses
modern protocols where they are available: the kitty keyboard protocol, synchronized output, styled
underlines, hyperlinks, clipboard access, color scheme change notifications, and more.

**Strengths:**

- Feature detection by querying the terminal, so the same code works on Linux, macOS, and Windows.
- Broad support for recent terminal extensions that most libraries don't expose.
- Works inline, on the alternate screen, or as a plain renderer over any `Write`.

uncurses sits below Ratatui's widgets: it plays the role of a backend such as crossterm, plus a
diffing renderer, with no widgets or layout of its own. Its cell and screen model is inspired by
Ultraviolet. The `uncurses-ratatui` crate is a Ratatui backend built on uncurses, so you can use
both together. Its first release was in May 2026.

- [GitHub](https://github.com/aymanbagabas/uncurses)
- [Crate](https://crates.io/crates/uncurses)
- [Documentation](https://docs.rs/uncurses)
- [Website](https://uncurses.org)

## Building on Ratatui

If you like Ratatui's rendering but want more structure, frameworks built on top of it may be a
better fit than switching libraries. [tui-realm](https://github.com/veeso/tui-realm), for example,
adds React and Elm inspired components, focus handling, and event routing on top of Ratatui widgets.
See the [third-party widgets showcase](/showcase/third-party-widgets/) for more.
