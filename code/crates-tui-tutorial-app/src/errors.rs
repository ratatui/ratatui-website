use color_eyre::{
    config::{EyreHook, HookBuilder, PanicHook},
    eyre::{self, Result},
};

use crate::tui;

pub fn install_hooks() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .into_hooks();

    install_color_eyre_panic_hook(panic_hook);
    install_eyre_hook(eyre_hook)?;

    Ok(())
}

fn install_color_eyre_panic_hook(panic_hook: PanicHook) {
    let panic_hook = panic_hook.into_panic_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        if let Err(err) = tui::restore() {
            println!("Unable to restore terminal: {err:?}");
        }
        panic_hook(panic_info);
    }));
}

fn install_eyre_hook(eyre_hook: EyreHook) -> color_eyre::Result<()> {
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        tui::restore().unwrap();
        eyre_hook(error)
    }))?;
    Ok(())
}
