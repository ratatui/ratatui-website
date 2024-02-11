// ANCHOR: main
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    println!("Sleeping for 5 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    Ok(())
}
// ANCHOR_END: main
