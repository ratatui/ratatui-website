# Tutorial

In this tutorial, we will be creating an application that gives the user a simple interface to enter key-value pairs, which will be printed to `stdout` in valid json.

## Initialization

Go ahead and create a new rust project with

```sh
cargo init ratatui-example-project
```
and put the following in the `Cargo.toml`:
```
{{#include ../../ratatui-book-tutorial-project/Cargo.toml:7:}}
```
or whatever is the latest version of these libraries


## Filestructure

Now create two more files inside of `src` so it looks like this:

```
src
| - main.rs
| - ui.rs
| - app.rs
```

This follows a common approach to small applications in ratatui, where we have an application state file, a ui functions file, and the main file to tie it all together.

