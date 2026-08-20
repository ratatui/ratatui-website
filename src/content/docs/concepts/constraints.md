---
title: Constraint
---

In Ratatui, users can use the `Layout` struct to allocate space which can be filled in with the
rendered content from widgets.

An area is represented by a `Rect`, which contains `x`, `y`, `width` and `height`. In the callback
function, `Frame` has a method called `.size()` that currents the user's current terminal size.

```rust
terminal.draw(|frame| {
    let area = frame.size();
    // calculate areas here
    frame.render(widget1, area1);
    frame.render(widget2, area2);
    frame.render(widget3, area3);
})?;
```

As an TUI application developer, you are now tasked with figuring out how to place the different
widgets in the different areas.

Since Ratatui uses an immediate mode rendering, you have access to size of the user's terminal every
frame, and you can recalculate a layout every frame.

For a given area and a set of constraints, the result is cached in an `LruCache`. You can increase
the size of this cache too, by calling `Layout::init_cache(new_size)`. So, you can use `Layout` and
constraints with abandon.

Ratatui uses a constraint solver (`cassowary`) with a set of constraints that help build commonly
used layouts. The basic idea is that you pass an array of "constraints" to the layout and you'll get
back an array of areas that represent segments corresponding to those constraints. Here's a simple
example to split an area in half horizontally.

```rust
let [area1, area2] = area.split(&Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
```

It is important to know that this layout is along just 1 direction, and is not 2 dimensional. You
have to choose if you want to split an area vertically or horizontally.

It is helpful to know a little bit about how the constraint solver works to figure out how to get
the layout you want.

There are 7 kinds of constraints in Ratatui:

1. `Constraint::Min(v)`: Limits the minimum size of the segment to user provided `v` but tries to
   expand the segment as much as possible without violating any other constraints
2. `Constraint::Max(v)`: Limits the maximum size of the segment to user provided `v` without
   violating any other constraints
3. `Constraint::Length(v)`: Allocates `v` pixels
4. `Constraint::Percentage(v)`: Allocates `v %` of total available area
5. `Constraint::Ratio(n, d)`: Allocates `v` percentage of available area
6. `Constraint::Fill(v)`: Grows to fills any excess area

As a pop quiz, can you guess what the following will do?

```rust
let [main, input] = area.split(&Layout::vertical([Constraint::Min(0), Constraint::Length(3)]);
```

When thinking which constraints to use allocate space in the available area, it is important to
realize that there are 3 situations that might arise.

1. There's exactly enough space to allocate the area you requested, e.g. you request for two 15 px
   wide length segments in a 50 px wide area:
2. There's more space than what you asked for, e.g. you request for two 25 px wide length segments
   in a 50 px wide area:
3. There's less space than what you asked for, e.g. you request for two 50 px wide length segments
   in a 50 px wide area:

In the first case, it is obvious what the algorithm should do, i.e. allocate what you asked for:

```kroki type=svgbob
<---------------------50 px---------------------->
┌───────────────────────┐┌───────────────────────┐
│      Length(25)       ││      Length(25)       │
│         25 px         ││         25 px         │
└───────────────────────┘└───────────────────────┘
```

In the second case, however, what should the layout algorithm do? In Ratatui [v0.26.0], new flex
modes were added that help allocate not just the segments, but also the excess space.

[v0.26.0]: https://github.com/ratatui-org/ratatui/releases/tag/v0.26.0

In this second case, in each of the `Flex` modes, this is how it behaves:

**`Flex::Start` (new default)**:

The segments are allocated at the start.

```kroki type=svgbob
<---------------------50 px---------------------->
┌─────────────┐┌─────────────┐┌                  ┐
│ Length(15)  ││ Length(15)  │
│    15 px    ││    15 px    │        20 px
└─────────────┘└─────────────┘└                  ┘
```

**`Flex::Center`**:

The segments are allocated at the center.

```kroki type=svgbob
<---------------------50 px---------------------->
┌        ┐┌─────────────┐┌─────────────┐┌        ┐
          │ Length(15)  ││ Length(15)  │
   10 px  │    15 px    ││    15 px    │   10 px
└        ┘└─────────────┘└─────────────┘└        ┘
```

**`Flex::End`**:

The segments are allocated at the end.

```kroki type=svgbob
<---------------------50 px---------------------->
┌                  ┐┌─────────────┐┌─────────────┐
                    │ Length(15)  ││ Length(15)  │
        20 px       │    15 px    ││    15 px    │
└                  ┘└─────────────┘└─────────────┘
```

**`Flex::SpaceAround`**:

The segments are allocated with spacers around each segment.

```kroki type=svgbob
<---------------------50 px---------------------->
┌     ┐┌─────────────┐┌    ┐┌─────────────┐┌     ┐
       │ Length(15)  │      │ Length(15)  │
 7 px  │    15 px    │ 6 px │    15 px    │ 7 px
└     ┘└─────────────┘└    ┘└─────────────┘└     ┘
```

**`Flex::SpaceBetween`**:

The segments are allocated with spacers in between each segment.

```kroki type=svgbob
<---------------------50 px---------------------->
┌─────────────┐┌                  ┐┌─────────────┐
│ Length(15)  │                    │ Length(15)  │
│    15 px    │        20 px       │    15 px    │
└─────────────┘└                  ┘└─────────────┘
```

**`Flex::Legacy` (old default)**:

For backwards compatibility reasons, we also support `Flex::Legacy` which will violate one or more
of the constraints provided by the user.

```kroki type=svgbob
<---------------------50 px---------------------->
┌─────────────┐┌─────────────────────────────────┐
│ Length(15)  ││           Length(15)            │
│    15 px    ││              35 px              │
└─────────────┘└─────────────────────────────────┘
```

In the third case, there really is no good way to solve layout problem. Most users typically will
not choose a layout like this on purpose, but if they do, the Ratatui Layout solver will try to do
its best to come up with a reasonable solution.

```kroki type=svgbob
<---------------------50 px---------------------->
┌───────────────────────┐┌───────────────────────┐
│      Length(50)       ││      Length(50)       │
│         25 px         ││         25 px         │
└───────────────────────┘└───────────────────────┘
```

With the new flex modes, you can center a `Rect` using the following:

```rust
let [center] = area.split(&Layout::vertical([Constraint::Percentage(50)]).flex(Flex::Center);
```
