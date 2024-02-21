// ANCHOR: main
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    println!("Spawning a task that sleeps 5 seconds...");

    let mut tasks = vec![];
    for i in 0..10 {
        tasks.push(tokio::spawn(async move {
            println!("Sleeping for 5 seconds in a tokio task {i}...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            i
        }));
    }

    println!("Getting return values from tasks...");
    while let Some(task) = tasks.pop() {
        let return_value_from_task = task.await?;
        println!("Got i = {return_value_from_task}");
    }

    Ok(())
}
// ANCHOR_END: main
