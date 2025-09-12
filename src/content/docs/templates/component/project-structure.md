---
title: Project Structure
sidebar:
  order: 1
---

The rust files in the `component` project are organized as follows:

```bash
$ tree
.
├── build.rs
└── src
    ├── action.rs
    ├── app.rs
    ├── cli.rs
    ├── components
    │   ├── fps.rs
    │   └── home.rs
    ├── components.rs
    ├── config.rs
    ├── errors.rs
    ├── logging.rs
    ├── main.rs
    └── tui.rs
```

Once you have set up the project, you shouldn't need to change the contents of anything outside the
`components` folder.

Let's discuss the contents of the files in the `src` folder first, how these contents of these files
interact with each other and why they do what they are doing.
