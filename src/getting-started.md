# Getting Started

In this section, we will explain how to install Ratatui and how to write a "hello world" demo program.
If you are already familiar with TUI-rs and wish to migrate your project, you will want to head over
to our [dedicated guide](./migrating-from-tui.md) on migrating from TUI-rs instead.

## Installation

To install Ratatui, you can simply run `cargo add ratatui` while inside your Cargo project directory.
This will add the newest release of Ratatui as a dependency for your project. You will also have to
import the backend of your choice by running `cargo add <crossterm|termion|termwiz>`. If you are
unsure of which backend to use at this point, read the "Choosing a Backend" section below.

Once you have added the dependencies, you are ready to start working on your TUI program!

## Choosing a Backend

Ratatui supports three "backends" - that is to say, the libraries that enable Ratatui (and you) to
interact with the terminal emulator program. These backends provide functionality for registering
keypresses, moving the cursor around the screen, and colorizing/stylizing text, among other things.
The currently-supported backends are Crossterm, Termion, and Termwiz.

The backend that you choose has some implications for the way you must structure your project, but
don't get intimidated by the options - they all do very similar things. To help you decide, we've come
up with some questions that you should ask yourself when choosing a backend, and our recommendations
for different use cases. The questions are listed in order of importance.

1. Do you need your program to Windows-compatible?
    - If yes, use Crossterm or Termwiz.
    - If no, use Crossterm or Termion.
2. Do you already have experience with one of the backends?
    - If yes, consider using the backend you are most familiar with, as long as it meets your platform
    compatibility needs.
    - If no, consider using Crossterm.
3. Do you intend for the TUI to be used primarily with the Wezterm terminal emulator?
    - If yes, use Termwiz.
    - If no, use Crossterm or Termion.

Though we try to make sure that all backends are fully-supported, the most commonly-used backend is
Crossterm. If you have no particular reason to use Termion or Termwiz, you will find it easiest
to learn Crossterm simply due to its popularity.

## Hello World
