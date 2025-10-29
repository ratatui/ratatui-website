---
title: Components/fps.rs
sidebar:
  order: 12
---

Here's an example of the `FpsCounter` component implemented in the template.

## State

The component has the following state:

1. `last_tick_update` is a `Instant` that tracks the last time the `tick` method was called.
2. `tick_count` is a `u32` that tracks the number of ticks in the last second.
3. `ticks_per_second` is a `f64` that tracks the number of ticks per second.
4. `last_frame_update` is a `Instant` that tracks the last time the `render` method was called.
5. `frames_per_second` is a `f64` that tracks the number of frames rendered per second.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FpsCounter {
    last_tick_update: Instant,
    tick_count: u32,
    ticks_per_second: f64,

    last_frame_update: Instant,
    frame_count: u32,
    frames_per_second: f64,
}
```

## Methods

### `impl FpsCounter`

In this `impl` block, we define the `new` method that creates a new `FpsCounter` component. We also
define some methods to calculate the `tick count`, `ticks_per_second` and more.

```rust

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_tick_update: Instant::now(),
            tick_count: 0,
            ticks_per_second: 0.0,
            last_frame_update: Instant::now(),
            frame_count: 0,
            frames_per_second: 0.0,
        }
    }

    fn app_tick(&mut self) -> Result<()> {
        self.tick_count += 1;
        let now = Instant::now();
        let elapsed = (now - self.last_tick_update).as_secs_f64();
        if elapsed >= 1.0 {
            self.ticks_per_second = self.tick_count as f64 / elapsed;
            self.last_tick_update = now;
            self.tick_count = 0;
        }
        Ok(())
    }

    fn render_tick(&mut self) -> Result<()> {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = (now - self.last_frame_update).as_secs_f64();
        if elapsed >= 1.0 {
            self.frames_per_second = self.frame_count as f64 / elapsed;
            self.last_frame_update = now;
            self.frame_count = 0;
        }
        Ok(())
    }
}
```

### `impl Component`

Lastly, we implement the `Component` trait for the `FpsCounter` component. This allows us to use the
component.

```rust
impl Component for FpsCounter {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            _ => {}
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [top, _] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);
        let message = format!(
            "{:.2} ticks/sec, {:.2} FPS",
            self.ticks_per_second, self.frames_per_second
        );
        let span = Span::styled(message, Style::new().dim());
        let paragraph = Paragraph::new(span).right_aligned();
        frame.render_widget(paragraph, top);
        Ok(())
    }
}
```
