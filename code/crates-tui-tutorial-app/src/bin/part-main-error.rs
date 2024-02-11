// ANCHOR: main
fn main() -> color_eyre::Result<()> {
    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    });
    Ok(())
}
// ANCHOR_END: main
