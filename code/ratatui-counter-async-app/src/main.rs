// ANCHOR: all
mod tui;

use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{self, UnboundedSender};
use tui::Event;

// App state
struct App {
    counter: i64,
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
}

// App actions
// ANCHOR: action_enum
#[derive(Clone)]
pub enum Action {
    Tick,
    Increment,
    Decrement,
    NetworkRequestAndThenIncrement, // new
    NetworkRequestAndThenDecrement, // new
    Quit,
    Render,
    None,
}
// ANCHOR_END: action_enum

// App ui render function
fn ui(f: &mut Frame, app: &mut App) {
    let area = f.size();
    f.render_widget(
        Paragraph::new(format!(
            "Press j or k to increment or decrement.\n\nCounter: {}",
            app.counter,
        ))
        .block(
            Block::default()
                .title("ratatui async counter app")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center),
        area,
    );
}

// ANCHOR: get_action
fn get_action(_app: &App, event: Event) -> Action {
    match event {
        Event::Error => Action::None,
        Event::Tick => Action::Tick,
        Event::Render => Action::Render,
        Event::Key(key) => {
            match key.code {
                Char('j') => Action::Increment,
                Char('k') => Action::Decrement,
                Char('J') => Action::NetworkRequestAndThenIncrement, // new
                Char('K') => Action::NetworkRequestAndThenDecrement, // new
                Char('q') => Action::Quit,
                _ => Action::None,
            }
        }
        _ => Action::None,
    }
}
// ANCHOR_END: get_action

// ANCHOR: update
fn update(app: &mut App, action: Action) {
    match action {
        Action::Increment => {
            app.counter += 1;
        }
        Action::Decrement => {
            app.counter -= 1;
        }
        Action::NetworkRequestAndThenIncrement => {
            let tx = app.action_tx.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request
                tx.send(Action::Increment).unwrap();
            });
        }
        Action::NetworkRequestAndThenDecrement => {
            let tx = app.action_tx.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request
                tx.send(Action::Decrement).unwrap();
            });
        }
        Action::Quit => app.should_quit = true,
        _ => {}
    };
}
// ANCHOR_END: update

// ANCHOR: run
async fn run() -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel(); // new

    // ratatui terminal
    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);
    tui.enter()?;

    // application state
    let mut app = App {
        counter: 0,
        should_quit: false,
        action_tx: action_tx.clone(),
    };

    loop {
        let e = tui.next().await?;
        match e {
            tui::Event::Quit => action_tx.send(Action::Quit)?,
            tui::Event::Tick => action_tx.send(Action::Tick)?,
            tui::Event::Render => action_tx.send(Action::Render)?,
            tui::Event::Key(_) => {
                let action = get_action(&app, e);
                action_tx.send(action.clone())?;
            }
            _ => {}
        };

        while let Ok(action) = action_rx.try_recv() {
            // application update
            update(&mut app, action.clone());
            // render only when we receive Action::Render
            if let Action::Render = action {
                tui.draw(|f| {
                    ui(f, &mut app);
                })?;
            }
        }

        // application exit
        if app.should_quit {
            break;
        }
    }
    tui.exit()?;

    Ok(())
}
// ANCHOR_END: run

#[tokio::main]
async fn main() -> Result<()> {
    let result = run().await;

    result?;

    Ok(())
}
// ANCHOR_END: all
