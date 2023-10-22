use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings},
};

#[derive(Default)]
pub struct PomodoroTimer {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  elapsed_time: f64,
  total_time: f64,
  titles: Vec<String>,
}

impl PomodoroTimer {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Component for PomodoroTimer {
  fn init(&mut self, area: Rect) -> Result<()> {
    self.elapsed_time = 0.0;
    self.total_time = 60.0;
    self.titles = vec![];
    Ok(())
  }

  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {},
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let gauge = Gauge::default()
      .block(Block::default().title("Pomodoro Timer").borders(Borders::ALL))
      .style(Style::default().fg(Color::Yellow))
      .ratio(self.elapsed_time as f64 / self.total_time as f64);
    f.render_widget(gauge, area);
    Ok(())
  }
}
