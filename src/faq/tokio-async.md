# When should I use `tokio` and `async`/`await`?

`ratatui` isn't a native `async` library. So is it beneficial to use `tokio` or `async`/`await`?

As a user of `rataui`, there really is only one point of interface with the `ratatui` library and
that's the `terminal.draw(|f| ui(f))` functionality (the creation of widgets provided by `ratatui`
typically happens in `ui(f)`). Everything else in your code is your own to do as you wish.

Should `terminal.draw(|f| ui(f))` be `async`? Possibly. Rendering to the terminal buffer is
relatively fast, especially using the double buffer technique that only renders diffs that `ratatui`
uses. Creating of the widgets can also be done quite efficiently.

So one question you may ask is can we make `terminal.draw(|f| ui(f))` `async` ourselves? Yes, we
can. Check out <https://github.com/ratatui-org/ratatui-async-template/tree/v0.1.0> for an example.

The only other part related to `ratatui` that is beneficial to being `async` is reading the key
event inputs from `stdin`, and that can be made `async` with `crossterm`'s event-stream.

So the real question is what other parts of your app require `async` or benefit from being `async`?
If the answer is not much, maybe it is simpler to not use `async` and avoiding `tokio`.

Another way to think about it is, do you think your app would work better with 1 thread like this?

```svgbob
 ,-------------.
 |Get Key Event|
 `-----+-------'
       |
       |
 ,-----v------.
 |Update State|
 `-----+------'
       |
       |
   ,---v----.
   | Render |
   `--------'
```

Or would it work with 3 threads / `tokio` tasks like this:

```svgbob
    Render Thread       ┊         Event Thread             ┊     Main Thread
                        ┊                                  ┊
                        ┊      ,------------------.        ┊
                        ┊      |  Get Key Event   |        ┊
                        ┊      `--------+---------'        ┊
                        ┊               |                  ┊
                        ┊     ,---------v-----------.      ┊
                        ┊     | Map Event to Action |      ┊
                        ┊     `---------+-----------'      ┊
                        ┊               |                  ┊
                        ┊  ,------------V--------------.   ┊     ,-------------.
                        ┊  | Send Action on action_tx  |---┊---->| Recv Action |
                        ┊  `---------------------------'   ┊     `-----+-------'
                        ┊                                  ┊           |
,-------------------.   ┊                                  ┊  ,--------v--------.
| Recv on render_rx |<--┊----------------------------------┊--| Dispatch Action |
`--------+----------'   ┊                                  ┊  `--------+--------'
         |              ┊                                  ┊           |
,--------v---------.    ┊                                  ┊  ,--------v---------.
| Render Component |    ┊                                  ┊  |   Update State   |
`------------------'    ┊                                  ┊  `------------------'
```

The former can be done without any `async` code and the latter is the approach showcased in
[`ratatui-async-template`](https://github.com/ratatui-org/ratatui-async-template) with `tokio`.
