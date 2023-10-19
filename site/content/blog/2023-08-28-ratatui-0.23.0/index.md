+++
title = "From tui-rs to Ratatui: 6 Months of Cooking Up Rust TUIs"
date = "2023-08-28"
authors = ["Orhun Parmaksız (https://orhun.dev/)"]
+++

Reposted from <https://blog.orhun.dev/ratatui-0-23-0/>

Let's take a look at what is new in the new version of
"[**Ratatui**](https://github.com/ratatui-org/ratatui)" and how it became the successor of
[`tui-rs`](https://github.com/fdehau/tui-rs).

<!-- more -->

When we decided to fork `tui-rs` with a couple of people [in this
issue](https://github.com/fdehau/tui-rs/issues/654), we had never imagined we would come this far.
It has been 6 months since we started working on `ratatui` and today I am excited to once again
announce that just recently `ratatui` became the [official
successor](https://github.com/fdehau/tui-rs/commit/335f5a4563342f9a4ee19e2462059e1159dcbf25) of
`tui-rs`! This is a big improvement for the project and it truly shows the power of open source and
the greatness of the Rust community. I am thrilled to be a part of this project and make `ratatui`
the most of the terminal user interface lover's favorite crate. As of now, existing `tui-rs`
projects started to migrate to `ratatui` and we got overall positive feedback from the maintainers.
Also, new and very interesting `ratatui` projects are being created every day and being shared on
our [Discord](https://discord.gg/pMCEU9hNEj) / [Matrix](https://matrix.to/#/#ratatui:matrix.org).
Come join us and do not miss these new terminal apps and contribution opportunities!

On a personal note, `ratatui` is an amazing experience for me since I have met great people and had
the chance to work with them on this _kickass_ open source project. I also learned a ton of new
things about Rust and open source maintenance. On the other hand, I believe I vastly improved my
community-building and communication skills. At the end of the day, the hard work continues and we
are all human. It is important to not forget that. 🐻

Alright, without further ado, let's jump into what happened so far.

### Retrospective

- `14-08-2022`: [Future of `tui-rs`](https://github.com/fdehau/tui-rs/issues/654) discussion is created.
- `02-02-2023`: We created a Discord server to discuss the possibility of forking the project.
- `08-02-2023`: The author got back to us and proposed a plan for transferring the ownership.
- `14-02-2023`: We created a fork to continue development.
- `18-02-2023`: We had our first `ratatui` meeting. You can read the notes [here](https://github.com/ratatui-org/ratatui/discussions/63).
- `19-03-2023`: The first version of `ratatui` is released. ([`0.20.0`](https://github.com/ratatui-org/ratatui/releases/tag/v0.20.0))
- `01-04-2023`: We had another meeting. ([notes](https://github.com/ratatui-org/ratatui/discussions/123))
- `29-05-2023`: [`0.21.0`](https://github.com/ratatui-org/ratatui/releases/tag/v0.21.0) is released.
  ([blog post](https://blog.orhun.dev/ratatui-0-21-0/))
- `15-07-2023`: Our biggest meeting so far! ([notes](https://github.com/ratatui-org/ratatui/discussions/316))
- `17-07-2023`: [`0.22.0`](https://github.com/ratatui-org/ratatui/releases/tag/v0.22.0) is released.
  ([blog post](https://blog.orhun.dev/ratatui-0-22-0/))
- `07-08-2023`: The original author archived the `tui-rs` repository and `ratatui` became the
  official successor. 🎉
  - See the [related
    commit](https://github.com/fdehau/tui-rs/commit/335f5a4563342f9a4ee19e2462059e1159dcbf25) &
    [security advisory](https://rustsec.org/advisories/RUSTSEC-2023-0049.html).

Such an adventure we had! And the best part is, it continues... In that regard, I would especially
like to thank [Florian Dehau](https://github.com/fdehau) since he created `tui-rs` and made all of
this possible! 💖

Now, let's get to what's new in `ratatui`! Buckle up, we're about to cook up some terminal interfaces.

**Full changelog**: [https://github.com/ratatui-org/ratatui/releases/tag/v0.23.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.23.0)

---

### Coolify everything 😎

We already had a cool name and a logo, and now we have a cool description as well:

```diff
- ratatui: A Rust library to build rich terminal user interfaces or dashboards.
+ ratatui: A Rust library that's all about cooking up terminal user interfaces.
```

We also renamed our organization from `tui-rs-revival` to `ratatui-org`:

- [**https://github.com/ratatui-org/ratatui**](https://github.com/ratatui-org/ratatui)

---

### Barchart: horizontal bars

You can now render the bars horizontally for the `Barchart` widget. This is especially useful in
some cases to make more efficient use of the available space.

Simply use the `Direction` attribute for rendering horizontal bars:

```rs
let mut barchart = BarChart::default()
    .block(Block::default().title("Data1").borders(Borders::ALL))
    .bar_width(1)
    .group_gap(1)
    .bar_gap(0)
    .direction(Direction::Horizontal);
```

Here is an example of what you can do with the `Barchart` widget (see the bottom right for
horizontal bars):

![horizontal bars](https://user-images.githubusercontent.com/15311024/263077132-ff80b2c4-9d2d-480b-9a72-f3779688bec8.png)

---

### Voluntary skipping capability for Sixel

> [Sixel](https://en.wikipedia.org/wiki/Sixel) is a bitmap graphics format supported by terminals.
> "Sixel mode" is entered by sending the sequence `ESC+Pq`.
> The "String Terminator" sequence `ESC+\` exits the mode.

`Cell` widget now has a `set_skip` method that allows the cell to be skipped when copying (diffing)
the buffer to the screen. This is helpful when it is necessary to prevent the buffer from
overwriting a cell that is covered by an image from some terminal graphics protocol such as Sixel,
iTerm, Kitty, etc.

See the pull request for more information:
[https://github.com/ratatui-org/ratatui/pull/215](https://github.com/ratatui-org/ratatui/pull/215)

In this context, there is also an experimental image rendering crate:
[_ratatu-image_](https://github.com/benjajaja/ratatu-image)

![ratatu-image](https://raw.githubusercontent.com/benjajaja/ratatu-image/master/assets/Recording.gif)

---

### Table/List: Highlight spacing

We added a new property called `HighlightSpacing` to the `Table` and `List` widgets and it can be
optionally set via calling `highlight_spacing` function.

Before this option was available, selecting a row in the table when no row was selected previously
made the tables layout change (the same applies to unselecting) by adding the width of the
"highlight symbol" in the front of the first column. The idea is that we want this behaviour to be
configurable with this newly added option.

```rs
let list = List::new(items)
    .highlight_symbol(">>")
    .highlight_spacing(HighlightSpacing::Always);
```

Right now, there are 3 variants:

- `Always`: Always add spacing for the selection symbol column.
- `WhenSelected`: Only add spacing for the selection symbol column if a row is selected.
- `Never`: Never add spacing to the selection symbol column, regardless of whether something is
  selected or not.

---

### Table: support line alignment

```rs
let table = Table::new(vec![
        Row::new(vec![Line::from("Left").alignment(Alignment::Left)]),
        Row::new(vec![Line::from("Center").alignment(Alignment::Center)]),
        Row::new(vec![Line::from("Right").alignment(Alignment::Right)]),
    ])
    .widths(&[Constraint::Percentage(100)]);
```

Now results in:

```text
Left
       Center
               Right
```

---

### Scrollbar: optional track symbol

The track symbol in the `Scrollbar` is now optional, simplifying composition with other widgets. It
also makes it easier to use the `Scrollbar` in tandem with a block with special block characters.

One breaking change is that `track_symbol` needs to be set in the following way now:

```diff
-let scrollbar = Scrollbar::default().track_symbol("-");
+let scrollbar = Scrollbar::default().track_symbol(Some("-"));
```

It also makes it possible to render a custom track that is composed out of multiple differing track symbols.

---

### `symbols::scrollbar` module

The symbols and sets are moved from `widgets::scrollbar` to `symbols::scrollbar`. This makes it
consistent with the other symbol sets. We also made the `scrollbar` module private.

Since this is a breaking change, you need to update your code to add an import for
`ratatui::symbols::scrollbar::*` (or the specific symbols you need).

---

### Alpha releases

The alpha releases (i.e. pre-releases) are created \*every Saturday\* and they are automated with
the help of [this GitHub Actions
workflow](https://github.com/ratatui-org/ratatui/blob/main/.github/workflows/cd.yml). This is
especially useful if you want to test `ratatui` or use unstable/experimental features before we hit
a stable release.

![alpha releases](./ratatui-alpha-releases.png)

The versioning scheme is `v<version>-alpha.<num>`, for example:
[v0.22.1-alpha.2](https://github.com/ratatui-org/ratatui/releases/tag/v0.22.1-alpha.2)

Additionally, see the following issue for possible contributions in the context of alpha releases
and documentation:
[https://github.com/ratatui-org/ratatui/issues/412](https://github.com/ratatui-org/ratatui/issues/412)

---

### Example GIFs

We added GIFs for each example in the `examples/` directory and added a `README.md` for preview.
This should make it easier to see what each example does without having to run it.

See:
[**https://github.com/ratatui-org/ratatui/blob/main/examples/README.md**](https://github.com/ratatui-org/ratatui/blob/main/examples/README.md)

One thing to note here is that we used [vhs](https://github.com/charmbracelet/vhs) for generating
GIFs from a set of instructions. For example:

```ini
# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/demo.tape`
Output "target/demo.gif"
Set Theme "OceanicMaterial"
Set Width 1200
Set Height 1200
Set PlaybackSpeed 0.5
Hide
Type "cargo run --example demo"
Enter
Sleep 2s
Show
Sleep 1s
Down@1s 12
Right
Sleep 4s
Right
Sleep 4s
```

Results in:

![ratatui
demo](https://camo.githubusercontent.com/a7c42a61ca3400569b2ebc1543a6f35e03fb09ba86e855137687b54f27b71038/68747470733a2f2f7668732e636861726d2e73682f7668732d7446305162755062744867556547307354566746722e676966)

We also host these GIFs at [https://vhs.charm.sh](https://vhs.charm.sh) but there is an issue about
moving everything to GitHub. If you are interested in contributing regarding this, see
[https://github.com/ratatui-org/ratatui/issues/401](https://github.com/ratatui-org/ratatui/issues/401)

---

### Common traits

With the help of [strum](https://crates.io/crates/strum) crate, we added `Display` and `FromStr`
implementation to enum types.

Also, we implemented common traits such as `Debug`, `Default`, `Clone`, `Copy`, `Eq`, `PartialEq`,
`Ord`, `PartialOrd`, `Hash` to the structs/enums where possible.

---

### Test coverage 🧪

`ratatui` now has [90% test coverage](https://app.codecov.io/gh/ratatui-org/ratatui)!

Shoutout to everyone who added tests/benchmarks for various widgets made this possible.

---

### No unsafe ⚠️

We now _forbid_ [unsafe code](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) in `ratatui`.
Also, see [this discussion](https://github.com/ratatui-org/ratatui/discussions/66) we had in the
past about using `unsafe` code for optimization purposes.

---

### The book 📕

We are working on a book for more in-depth `ratatui` documentation and usage examples, you can read
it from here:
[https://ratatui-org.github.io/ratatui-book/](https://ratatui-org.github.io/ratatui-book/)

Repository: [**https://github.com/ratatui-org/ratatui-book**](https://github.com/ratatui-org/ratatui-book)

---

### Other

- Expand serde attributes for `TestBuffer` for de/serializing the whole test buffer.
- Add weak constraints to make `Rect`s closer to each other in size.
- Simplify `Layout::split` function.
- Various bug fixes and improvements in Barchart, Block, Layout and other widgets.
- Add documentation to various widgets and improve existing documentation.
- Add examples for colors and modifiers.
- We created a Matrix bridge at [#ratatui:matrix.org](https://matrix.to/#/#ratatui:matrix.org).

---

## Endnote

Feel free to join our [Discord](https://discord.gg/pMCEU9hNEj) /
[Matrix](https://matrix.to/#/#ratatui:matrix.org) for chatting/getting help about `ratatui`. If you
are interested in contributing, check out our [contribution
guidelines](https://github.com/ratatui-org/ratatui/blob/main/CONTRIBUTING.md) and [open
issues](https://github.com/ratatui-org/ratatui/issues) for getting started!

If you have any crazy ideas about terminal user interfaces and/or in general about `ratatui`, go
ahead and [submit an issue](https://github.com/ratatui-org/ratatui/issues/new/choose)!

Lastly, shout out to these awesome people for their first contributions to the project:

- @[EdJoPaTo](https://github.com/EdJoPaTo)
- @[mhovd](https://github.com/mhovd)
- @[joshrotenberg](https://github.com/joshrotenberg)
- @[t-nil](https://github.com/t-nil)
- @[ndd7xv](https://github.com/ndd7xv)
- @[TieWay59](https://github.com/TieWay59)
- @[Valentin271](https://github.com/Valentin271)
- @[hasezoey](https://github.com/hasezoey)
- @[jkcdarunday](https://github.com/jkcdarunday)
- @[stappersg](https://github.com/stappersg)
- @[benjajaja](https://github.com/benjajaja)

_cya!_

🐭
