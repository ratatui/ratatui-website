---
title: JSON Editor
sidebar:
  order: 0
---

Now that we have covered some of the basics of a "hello world" and "counter" app, we are ready to
build and manage something more involved.

In this tutorial, we will be creating an application that gives the user a simple interface to enter
key-value pairs, which will be converted and printed to `stdout` in json. The purpose of this
application will be to give the user an interface to create correct json, instead of having to worry
about commas and brackets themselves.

Here’s a gif of what it will look like if you run this:

![Demo](https://vhs.charm.sh/vhs-5VaEPLZP2OlOxPPAIiLqbF.gif)

## Initialization

Go ahead and set up a new rust project with

```shell
cargo new ratatui-json-editor
```

and put the following in the `Cargo.toml`:

```toml
{{#include @code/ratatui-json-editor-app/Cargo.toml:10:}}
```

or the latest version of these libraries.

## Filestructure

Now create two files inside of `src/` so it looks like this:

```
src
├── main.rs
├── ui.rs
└── app.rs
```

This follows a common approach to small applications in `ratatui`, where we have a state file, a UI
file, and the main file to tie it all together.
