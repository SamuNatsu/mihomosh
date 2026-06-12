use comfy_table::{Cell, CellAlignment, Table};
use eyre::{Context, OptionExt, Result, eyre};
use tokio::{runtime::Runtime, task::JoinSet};

use crate::{
    cli::profile::{ProfileOptionalTarget, ProfileTarget},
    models::{profile::entity::Model as Profile, profile_manager::ProfileManager},
    utils::{
        dialog,
        log::{error, info, log, primary, success},
        tools,
    },
};

pub fn list() -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch all profiles
    let mut profile =
        rt.block_on(async { ProfileManager::get_instance().await.fetch_all().await })?;
    if profile.is_empty() {
        log!("No profiles. Please create first.");
        return Ok(());
    }

    // Sort
    profile.sort_by(|a, b| {
        // Name
        let cmp = a.name.cmp(&b.name);
        if cmp.is_ne() {
            return cmp;
        }

        // UUID
        a.uuid.cmp(&b.uuid)
    });

    // Partition
    let (local, remote) = profile
        .into_iter()
        .partition::<Vec<_>, _>(|a| a.r#type.is_local());

    // Print tables
    if !local.is_empty() {
        let mut table = Table::new();
        table
            .load_preset("  -- ==      --    ")
            .set_header(vec![
                Cell::new("UUID").set_alignment(CellAlignment::Center),
                "Name".into(),
                Cell::new("Updated At").set_alignment(CellAlignment::Center),
            ])
            .column_mut(2)
            .unwrap()
            .set_cell_alignment(CellAlignment::Center);
        local.iter().for_each(|v| {
            table.add_row(v.row());
        });

        primary!("Local Profile(s)");
        println!("{table}");
    }

    if !remote.is_empty() {
        let mut table = Table::new();
        table.load_preset("  -- ==      --    ").set_header(vec![
            Cell::new("UUID").set_alignment(CellAlignment::Center),
            "Name".into(),
            Cell::new("Updated At").set_alignment(CellAlignment::Center),
            Cell::new("Expired At").set_alignment(CellAlignment::Center),
            "Usage".into(),
        ]);
        table
            .column_mut(2)
            .unwrap()
            .set_cell_alignment(CellAlignment::Center);
        table
            .column_mut(3)
            .unwrap()
            .set_cell_alignment(CellAlignment::Center);
        remote.iter().for_each(|v| {
            table.add_row(v.row());
        });

        if !local.is_empty() {
            println!();
        }
        primary!("Remote Profile(s)");
        println!("{table}");
    }

    // Done
    Ok(())
}

pub fn create() -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Render default profile
    let contents = Profile::default()
        .render()
        .wrap_err("failed to render default profile information")?;

    // Edit
    let contents = rt.block_on(async {
        tools::edit_contents(contents, "info.yaml")
            .await
            .wrap_err("failed to edit profile information")
    })?;

    // Parse model
    let profile = serde_saphyr::from_str_validate::<Profile>(&contents)
        .wrap_err("failed to parse profile information")?;

    // Ask user
    let prompt = format!("Are you sure to create the profile `{}`?", profile.name);
    let confirmed = dialog::confirm(&prompt, false).wrap_err("failed to show confirm dialog")?;

    // Do actions
    if confirmed {
        let model =
            rt.block_on(async { ProfileManager::get_instance().await.create(profile).await })?;
        success!(
            "Successfully created. New profile `{}` was assigned UUID `{}`.",
            model.name,
            model.uuid
        );
    } else {
        log!("Create skipped");
    }

    // Done
    Ok(())
}

pub fn delete(target: ProfileTarget) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch profile
    let profile = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;

    // Ask user
    let prompt = format!("Are you sure to delete the profile `{}`?", profile.name);
    let confirmed = dialog::confirm(&prompt, false).wrap_err("failed to show confirm dialog")?;

    // Do actions
    if confirmed {
        rt.block_on(async {
            ProfileManager::get_instance()
                .await
                .delete(profile.uuid)
                .await
        })?;
        success!("Successfully deleted");
    } else {
        log!("Delete skipped");
    }

    // Done
    Ok(())
}

pub fn update(target: ProfileOptionalTarget, no_reactivate: bool) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch profiles
    let profiles = if target.name.is_none() && target.uuid.is_none() {
        rt.block_on(async { ProfileManager::get_instance().await.fetch_all().await })?
    } else {
        rt.block_on(async {
            ProfileTarget {
                uuid: target.uuid.clone(),
                name: target.name.clone(),
            }
            .fetch_profile()
            .await
            .and_then(|v| match v {
                Some(v) => Ok(vec![v]),
                None => Err(eyre!("profile not found with given target")),
            })
        })?
    }
    .into_iter()
    .filter(|v| v.r#type.is_remote())
    .collect::<Vec<_>>();
    info!("Found {} profile(s) to be updated", profiles.len());

    // Skip empty
    if profiles.is_empty() {
        log!("Update skipped because of no need");
        return Ok(());
    }

    // Run async
    rt.block_on(async {
        // Create tasks
        let db = ProfileManager::get_instance().await;
        let mut tasks = JoinSet::new();
        for mut profile in profiles {
            tasks.spawn(async move {
                let prof = profile.clone();
                let ret = profile.update(db).await;
                (prof, ret)
            });
        }

        // Wait tasks
        while let Some(res) = tasks.join_next().await {
            let (profile, result) = res.wrap_err("failed to run task")?;
            match result {
                Ok(_) => success!(
                    "Profile `{}` with UUID `{}` updated",
                    profile.name,
                    profile.uuid
                ),
                Err(err) => error!(
                    "Failed to update profile `{}` with UUID `{}`: {}",
                    profile.name, profile.uuid, err
                ),
            }
        }
        Ok::<(), eyre::Report>(())
    })?;

    // Reactivate profile
    if !no_reactivate {
        todo!();
    }

    // Done
    Ok(())
}
