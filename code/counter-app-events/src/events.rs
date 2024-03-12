use std::{sync::mpsc, thread};

use crate::{app::App, tui::Tui};
use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::{self, Event, KeyEventKind};
use tracing::{debug, info, instrument};

#[derive(Debug)]
pub struct EventLoop {
    app: App,
    event_tx: mpsc::Sender<AppEvent>,
    event_rx: mpsc::Receiver<AppEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    Crossterm(Event),
    Redraw,
    Quit,
}

impl EventLoop {
    pub fn new(mut app: App) -> Self {
        let (event_tx, event_rx) = mpsc::channel();
        app.event_tx = Some(event_tx.clone());
        Self {
            app,
            event_tx,
            event_rx,
        }
    }

    #[instrument(skip(self, terminal))]
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        info!("starting event loop");
        let event_tx = self.event_tx.clone();
        let _ = thread::spawn(move || crossterm_event_thread(event_tx));
        self.draw(terminal)?;
        while let Ok(event) = self.event_rx.recv() {
            debug!(?event, "handling event");
            match event {
                AppEvent::Crossterm(event) => self.handle_crossterm(event)?,
                AppEvent::Redraw => self.draw(terminal)?,
                AppEvent::Quit => break,
            }
        }
        info!("finished event loop");
        Ok(())
    }

    #[instrument(skip(self))]
    fn handle_crossterm(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.app.handle_key_press(key_event).wrap_err_with(|| {
                    format!("handling key event failed:\n{key_event:#?}")
                })?;
            }
            _ => self.send_event(AppEvent::Redraw),
        }
        Ok(())
    }

    #[instrument(skip(self, terminal))]
    fn draw(&mut self, terminal: &mut Tui) -> Result<()> {
        debug!("drawing");
        terminal.draw(|frame| {
            frame.render_widget(&self.app, frame.size());
        })?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn send_event(&self, event: AppEvent) {
        debug!(?event, "sending event");
        let _ = self.event_tx.send(event);
    }
}

#[instrument(skip(event_tx))]
fn crossterm_event_thread(event_tx: mpsc::Sender<AppEvent>) {
    while let Ok(event) = event::read() {
        debug!(?event, "read crossterm event");
        let _ = event_tx.send(AppEvent::Crossterm(event));
    }
}
