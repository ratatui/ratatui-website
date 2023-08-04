# Tutorial
In this tutorial, we will be creating an application that gives the user a simple interface to enter key-value pairs, which will be converted and printed to `stdout` in json.

The purpose of this application will be to give the user an interface to create correct json,
instead of having to worry about commas and brackets themselves.

## Initialization
Go ahead and set up a new rust project with

```sh
cargo init ratatui-example-project
```
and put the following in the `Cargo.toml`:
```
{{#include ../../ratatui-book-tutorial-project/Cargo.toml:7:}}
```
or the latest version of these libraries


## Filestructure
Now create two files inside of `src/` so it looks like this:
```
src/
| - main.rs
| - ui.rs
| - app.rs
```
This follows a common approach to small applications in `ratatui`, where we have a state file, a UI file, and the main file to tie it all together.
