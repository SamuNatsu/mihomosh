use eyre::{Context, Result};
use tokio::runtime::Runtime;

use crate::{
    cli::config::ConfigCommand,
    models::config::Config,
    utils::{
        dialog,
        log::{log, success},
        tools,
    },
};

pub fn handle(cmd: ConfigCommand) -> Result<()> {
    match cmd {
        ConfigCommand::Show => show(),
        ConfigCommand::Edit { no_reactivate } => edit(no_reactivate),
        ConfigCommand::Reset { no_reactivate } => reset(no_reactivate),
    }
}

fn show() -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Render configurations
    let contents = Config::get_instance()
        .lock()
        .unwrap()
        .render()
        .wrap_err("failed to render configurations")?;

    // View
    rt.block_on(async {
        tools::view_contents(&contents, "config.yaml")
            .await
            .wrap_err("failed to show configurations")
    })
}

fn edit(no_reactivate: bool) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Render configurations
    let contents = Config::get_instance()
        .lock()
        .unwrap()
        .render()
        .wrap_err("failed to render configurations")?;

    // Edit
    let contents = rt.block_on(async {
        tools::edit_contents(&contents, "config.yaml")
            .await
            .wrap_err("failed to edit configurations")
    })?;

    // Parse model
    let config = serde_saphyr::from_str_validate::<Config>(&contents)
        .wrap_err("failed to parse configurations")?;

    // Ask user
    let confirmed = dialog::confirm("Are you sure to edit the configurations?", false)
        .wrap_err("failed to show confirm dialog")?;

    // Do actions
    if confirmed {
        Config::commit(config)?;
        success!("Successfully edited");
    } else {
        log!("Edit skipped");
    }

    // Reactivate profile
    if !no_reactivate {
        todo!();
    }

    // Done
    Ok(())
}

fn reset(no_reactivate: bool) -> Result<()> {
    // Ask user
    let confirmed = dialog::confirm("Are you sure to reset the configurations?", false)
        .wrap_err("failed to show confirm dialog")?;

    // Do actions
    if confirmed {
        Config::reset().wrap_err("failed to reset configurations")?;
        success!("Successfully reset");
    } else {
        log!("Reset skipped");
    }

    // Reactivate profile
    if !no_reactivate {
        todo!();
    }

    // Done
    Ok(())
}
