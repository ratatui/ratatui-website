---
title: Comparison of Backends
sidebar:
  order: 1
  label: Comparison
---

:::tip[TLDR]

Choose [Crossterm](https://crates.io/crates/crossterm) for most tasks.

:::

Ratatui interfaces with the terminal emulator through its "backends". These are powerful libraries
that grant `ratatui` the ability to capture keypresses, maneuver the cursor, style the text with
colors and other features. As of now, `ratatui` supports four backends:

- [Crossterm](https://crates.io/crates/crossterm)
- [Termion](https://crates.io/crates/termion)
- [Termwiz](https://crates.io/crates/termwiz)
- [Termina](https://crates.io/crates/termina)

Selecting a backend does influence your project's structure, but the core functionalities remain
consistent across all options. Here's a flowchart that can help you make your decision.

```mermaid
graph TD;
    Q1[Is the TUI only for Wezterm users?]
    Q2[Is Windows compatibility important?]
    Q3[Are you familiar with Crossterm?]
    Q4[Are you familiar with Termion?]
    Q5[Are you familiar with Termina?]
    Crossterm
    Termwiz
    Termion
    Termina

    Q1 -->|Yes| Termwiz
    Q1 -->|No| Q2
    Q2 -->|Yes| Crossterm
    Q2 -->|No| Q3
    Q3 -->|Yes| Crossterm
    Q3 -->|No| Q4
    Q4 -->|Yes| Termion
    Q4 -->|No| Q5
    Q5 -->|Yes| Termina
    Q5 -->|No| Crossterm
```

Though we try to make sure that all backends are fully-supported, the most commonly-used backend is
Crossterm. If you have no particular reason to use Termion, Termwiz, or Termina, you will find it
easiest to learn Crossterm simply due to its popularity.
