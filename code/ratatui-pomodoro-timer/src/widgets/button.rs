use std::{error::Error, io, ops::ControlFlow, time::Duration};

use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

/// A custom widget that renders a button with a label, theme and state.
#[derive(Debug, Clone)]
pub struct Button<'a> {
  label: Line<'a>,
  theme: Theme,
  state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
  Normal,
  Selected,
  Active,
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
  text: Color,
  background: Color,
  highlight: Color,
  shadow: Color,
}

const BLUE: Theme = Theme {
  text: Color::Rgb(16, 24, 48),
  background: Color::Rgb(48, 72, 144),
  highlight: Color::Rgb(64, 96, 192),
  shadow: Color::Rgb(32, 48, 96),
};

const RED: Theme = Theme {
  text: Color::Rgb(48, 16, 16),
  background: Color::Rgb(144, 48, 48),
  highlight: Color::Rgb(192, 64, 64),
  shadow: Color::Rgb(96, 32, 32),
};

const GREEN: Theme = Theme {
  text: Color::Rgb(16, 48, 16),
  background: Color::Rgb(48, 144, 48),
  highlight: Color::Rgb(64, 192, 64),
  shadow: Color::Rgb(32, 96, 32),
};

/// A button with a label that can be themed.
impl<'a> Button<'a> {
  pub fn new<T: Into<Line<'a>>>(label: T) -> Button<'a> {
    Button { label: label.into(), theme: BLUE, state: State::Normal }
  }

  pub fn theme(mut self, theme: Theme) -> Button<'a> {
    self.theme = theme;
    self
  }

  pub fn state(mut self, state: State) -> Button<'a> {
    self.state = state;
    self
  }
}

impl<'a> Widget for Button<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let (background, text, shadow, highlight) = self.colors();
    buf.set_style(area, Style::new().bg(background).fg(text));

    // render top line if there's enough space
    if area.height > 2 {
      buf.set_string(area.x, area.y, "▔".repeat(area.width as usize), Style::new().fg(highlight).bg(background));
    }
    // render bottom line if there's enough space
    if area.height > 1 {
      buf.set_string(
        area.x,
        area.y + area.height - 1,
        "▁".repeat(area.width as usize),
        Style::new().fg(shadow).bg(background),
      );
    }
    // render label centered
    buf.set_line(
      area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
      area.y + (area.height.saturating_sub(1)) / 2,
      &self.label,
      area.width,
    );
  }
}

impl Button<'_> {
  fn colors(&self) -> (Color, Color, Color, Color) {
    let theme = self.theme;
    match self.state {
      State::Normal => (theme.background, theme.text, theme.shadow, theme.highlight),
      State::Selected => (theme.highlight, theme.text, theme.shadow, theme.highlight),
      State::Active => (theme.background, theme.text, theme.highlight, theme.shadow),
    }
  }
}
