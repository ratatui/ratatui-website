# main.rs

Putting it all together, we have the `main.rs` function:

```rust
{{#include ./ratatui-book-tutorial-counter-app/src/main.rs}}
```

Because we call `tui.events.next()` in a loop, it blocks until there's an event generated. If
there's a key press, the state updates and the UI is refreshed. If there's no key press, a `Tick`
event is generated every 250 milliseconds, which causes the UI to be refreshed.

This is what it looks like in practice to:

- Run the TUI
- Wait 2.5 seconds
- Press `j` 5 times
- Wait 2.5 seconds
- Press `k` 5 times
- Wait 2.5 seconds
- Press `q`

<!--

```
Set Shell zsh
Sleep 2.5s
Type "cargo run"
Enter
Sleep 5s
Type "jjjjj"
Sleep 500ms
Type "kkkkk"
Sleep 5s
Type "q"
Sleep 2.5s
```

-->

![](https://user-images.githubusercontent.com/1813121/263404720-41bd81a0-4eec-479c-9333-44363a183613.gif)
