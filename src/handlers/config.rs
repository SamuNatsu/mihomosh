use std::fs::File;

use eyre::{Context, Result};

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
        ConfigCommand::Show => show()?,
        ConfigCommand::Edit { no_reactivate } => edit(no_reactivate)?,
        ConfigCommand::Reset { no_reactivate } => reset(no_reactivate)?,
    }
    Ok(())
}

fn show() -> Result<()> {
    // Render configurations
    let contents = Config::get_instance()
        .render()
        .wrap_err("fail to render configurations")?;

    // View
    tools::view_contents(&contents, "config.yaml").wrap_err("fail to show configurations")
}

fn edit(no_reactivate: bool) -> Result<()> {
    // Render configurations
    let contents = Config::get_instance()
        .render()
        .wrap_err("fail to render configurations")?;

    // Edit
    let contents =
        tools::edit_contents(&contents, "config.yaml").wrap_err("fail to edit configurations")?;

    // Parse model
    let config = serde_saphyr::from_str_validate::<Config>(&contents)
        .wrap_err("fail to parse configurations")?;

    // Ask user
    let confirmed = dialog::confirm("Are you sure to edit the configurations?", false)
        .wrap_err("fail to show confirm dialog")?;

    // Do actions
    if confirmed {
        let path = Config::get_path();
        let file = File::create(path)
            .wrap_err_with(|| format!("fail to create file `{}`", path.display()))?;
        serde_json::to_writer(&file, &config)
            .wrap_err_with(|| format!("fail to serialize value to file `{}`", path.display()))?;
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
        .wrap_err("fail to show confirm dialog")?;

    // Do actions
    if confirmed {
        Config::reset().wrap_err("fail to reset configurations")?;
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
