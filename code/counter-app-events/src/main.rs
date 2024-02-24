mod app;
mod errors;
mod events;
mod logging;
mod tui;

use app::App;
use clap::Parser;
use color_eyre::Result;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let args = CliArgs::parse();
    let logs = logging::init_logger(args.log_level)?;
    let app = App::new(logs);
    let mut event_loop = events::EventLoop::new(app);
    let mut terminal = tui::init()?;
    event_loop.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}

#[derive(Debug, Parser)]
struct CliArgs {
    /// Set the log level
    ///
    /// Possible values: trace, debug, info, warn, error (or more complex filters like "app=debug,app::module=trace")
    #[clap(long = "log", short)]
    log_level: Option<String>,
}
