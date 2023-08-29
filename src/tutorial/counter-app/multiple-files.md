# Multiple Files

At the moment, we have everything in just one file. However, this can be impractical if we want to
expand our app further.

Let's start by creating a number of different files to represent the various concepts we covered in
the previous section:

```bash
$ tree .
├── Cargo.toml
├── LICENSE
└── src
   ├── app.rs
   ├── event.rs
   ├── lib.rs
   ├── main.rs
   ├── tui.rs
   ├── ui.rs
   └── update.rs
```
