---
title: Errors
---

You now have a `tui` module that restores the state of the terminal at the end of `main`.

But in addition to that, you want to make sure that even when the application panics, you restore
the state of the terminal back to a normal working state. You will also want to print the error to
the terminal to that the user can see what went wrong.

Rust has a built-in function called [`set_hook`] to set a panic hook. Additionally, `color_eyre` has
some ready to install hooks to you can leverage for better panics.

[`set-hook`]: https://doc.rust-lang.org/std/panic/fn.set_hook.html

Putting that together along with restoring the terminal backend state might look something like
this:

```rust
    let (panic_hook, _) = color_eyre::config::HookBuilder::default().into_hooks();
    let panic_hook = panic_hook.into_panic_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        if let Err(err) = crate::tui::restore() {
            log::error!("Unable to restore terminal: {err:?}");
        }
        panic_hook(panic_info);
    }));
```

You can customize the output of the panic hook in a number of different ways. For example, with
[`human-panic`], you can autogenerate a log file that contains the stacktrace that a user can submit
to you for further investigation.

[`human-panic`]: https://github.com/rust-cli/human-panic

Here's the code using color_eyre to set a panic hook. Put the contents of this file into
`src/errors.rs`:

```rust
{{#include @code/crates-tui-tutorial-app/src/errors.rs}}
```

</details>

Let's update `main.rs` to the following:

```diff lang="rust"
  mod crates_io_api_helper;
+ mod errors;
  mod tui;

  #[tokio::main]
  async fn main() -> color_eyre::Result<()> {
+     errors::install_hooks()?;

      let mut tui = tui::init()?;

      tui.draw(|frame| {
          frame.render_widget(
              ratatui::widgets::Paragraph::new("hello world"),
              frame.size(),
          );
+         // panic!("Oops. Something went wrong!");
      })?;
      tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

      tui::restore()?;

      Ok(())
  }
```

:::note[Homework]

Experiment with uncommenting the `panic!` in the code and see what happens. Try to run the code with
`panic!` and with and without the `errors::install_hooks()` call.

:::

:::tip

If your terminal is in a messed up state, you can type `reset` and hit enter in the terminal to
reset your terminal state at any time.

:::

Your file structure should now look like this:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
   ├── crates_io_api_helper.rs
   ├── errors.rs
   ├── main.rs
   └── tui.rs
```
