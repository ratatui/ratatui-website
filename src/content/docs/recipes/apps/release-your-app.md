---
title: Releasing Your App
sidebar:
  order: 10
  label: Releasing Your App
---

At some point you'll probably want to release your app to the public. Here are some guidelines that
capture some advice regularly given on apps that are put in the showcase channel of the Ratatui
Discord server / forum. This is not a comprehensive list, but it should help you get started.

## Release Checklist

- Commit your `cargo.lock` file.
- Add `--locked` to your `cargo install` command. This makes sure that your users will install the
  same version of your dependencies that you used to build your app.
- Add a `README.md` file to your project. This file should contain:
  - A short description of your app.
  - A list of hotkeys and their functions.
  - A screenshot of your app in action.
- Add a `LICENSE` file to your project. This file should contain the license you are using for your
  app. The most common licenses are MIT and Apache 2.0. You can use [ChooseALicense] to help you
  choose a license.
- Enable additional [optimizations] in your `Cargo.toml` file.
- Consider using `Release-plz` to automate your GitHub releases. This makes doing a release as easy
  as clicking merge on an automatically generated PR.
- Submit your app to the [Awesome Ratatui] list, the [Ratatui Discord], and the [Ratatui Forum].
  This is a great way to get feedback on your app and to get it in front of more users.

[ChooseALicense]: https://choosealicense.com/
[Awesome Ratatui]: https://github.com/ratatui-org/awesome-ratatui
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
[Ratatui Forum]: https://forum.ratatui.rs
[optimizations]: https://ratatui.rs/recipes/apps/release-your-app/#optimizations

## Screenshots

Don't forget to add a screenshot / gif of your app in action. This will help users understand what
your app does and how it looks. We recommend using a tool like [VHS] to automate the process of
creating screenshots and gifs. See the `.tape` files in the [Ratatui repository] for examples of how
we use VHS to create screenshots and gifs for our all the examples and widgets.

Some tips for creating good screenshots:

TL;DR: Use [VHS], `Set Width 1200`, `Set Theme "Aardvark Blue"`, `Sleep 2s`, `Hide`/`Show` around
CLI command to run the app,

- Use a dark background for your terminal. Don't flashbang devs with light screens.
- Choose a good color scheme that shows off your app. Aardvark Blue is the choice we use in Ratatui,
  but Catppuccin and Solarized are also good options. There are many bad options :D
- Use an image width 1200px or less. This works well with the GitHub UI and the crates.io page.
- Use a font size that is similar that when rendered is approximately the same size as the text on
  the github README. The default VHS font size is good here.
- Skip the command line in the screenshot by using the `Hide` and `Show` commands in VHS. This will
  make your screenshots look cleaner and more professional.
- Wait around 2 seconds after making some changes to let the viewier read and understand the
  changes. This is especially important for gifs with a lot of information changing on each screen.
- Avoid storing the screenshot / gifs in your repo. This tends to bloat the repo and make it harder
  to clone. Instead, use VHS to publish the screenshots (`vhs publish`), or store them in a PR
  comment, an image hosting service, or your own website.
- Try to get to the meat of your app as quickly as possible. Don't waste time on the loading screen
  or splash screen. This is especially important for gifs, where you have a limited amount of time
  to show off your app.

[VHS]: https://github.com/charmbracelet/vhs

## Optimizations

Make sure you enable additional compiler optimizations for the release build. This will help reduce
the size of the resulting binary. Add the following lines to your `Cargo.toml` file:

```toml
[profile.release]
codegen-units = 1 # Allows compiler to perform better optimization.
lto = true # Enables Link-time Optimization.
opt-level = "s" # Prioritizes small binary size. Use `3` if you prefer speed.
strip = true # Ensures debug symbols are removed.
```

### References

- [codegen-units]: Tweaks a tradeoff between compile times and compile time optimizations.
- [lto]: Enables Link-time Optimization.
- [opt-level]: Determines the focus of the compiler in optimizations. Use `3` to optimize
  performance, `z` to optimize for size, and `s` for something in-between.
- [strip]: Strip either symbols or debuginfo from a binary.

[codegen-units]: https://doc.rust-lang.org/cargo/reference/profiles.html#codegen-units
[lto]: https://doc.rust-lang.org/cargo/reference/profiles.html#lto
[opt-level]: https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level
[strip]: https://doc.rust-lang.org/cargo/reference/profiles.html#strip
