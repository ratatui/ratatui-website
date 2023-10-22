use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
  action::Action,
  config::Config,
  tui::{Event, Frame},
};

pub mod button;
