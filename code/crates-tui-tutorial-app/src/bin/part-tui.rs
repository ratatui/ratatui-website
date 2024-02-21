use crates_tui::tui;

// ANCHOR: main
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let mut tui = tui::init()?;

    tui.draw(|frame| {
        frame.render_widget(
            ratatui::widgets::Paragraph::new("hello world"),
            frame.size(),
        )
    })?;

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    tui::restore()?;

    Ok(())
}
// ANCHOR_END: main
