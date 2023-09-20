# Changing font size

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
