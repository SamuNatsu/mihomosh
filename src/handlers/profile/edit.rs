use eyre::{Context, OptionExt, Result};
use tokio::{fs, runtime::Runtime};

use crate::{
    cli::profile::{ProfileGlobalTarget, ProfileTarget},
    models::{
        profile::{self, entity::Model as Profile},
        profile_manager::{ProfileManager, fetch::GlobalTargetFetchResult},
    },
    utils::{
        dialog,
        log::{error, log, success},
        tools,
    },
};

pub fn info(target: ProfileTarget) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch profile
    let profile = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;
    let uuid = profile.uuid.clone();

    // Render profile
    let contents = profile
        .render()
        .wrap_err("failed to render profile information")?;

    // Edit
    let contents = rt.block_on(async {
        tools::edit_contents(&contents, "info.yaml")
            .await
            .wrap_err("failed to edit profile information")
    })?;

    // Parse model
    let mut profile = serde_saphyr::from_str_validate::<Profile>(&contents)
        .wrap_err("failed to parse profile information")?;
    profile.uuid = uuid;

    // Ask user
    let prompt = format!("Are you sure to edit the profile `{}`?", profile.name);
    let confirmed = dialog::confirm(&prompt, false).wrap_err("failed to show confirm dialog")?;

    // Do actions
    if confirmed {
        rt.block_on(async {
            ProfileManager::get_instance()
                .await
                .update_info(profile)
                .await
        })?;
        success!("Successfully edited");
    } else {
        log!("Edit skipped");
    }

    // Done
    Ok(())
}

pub fn data(target: ProfileTarget, no_reactivate: bool) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch profile
    let mut profile = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;

    // Edit
    let path = profile.get_data_path();
    let contents = rt.block_on(async {
        if path.is_file() {
            tools::edit_file(&path)
                .await
                .wrap_err("failed to edit profile data")
        } else {
            tools::edit_contents("", "profile.yaml")
                .await
                .wrap_err("failed to edit profile data")
        }
    })?;

    // Validate contents
    if contents.trim().is_empty() {
        error!("Profile data CANNOT be empty");
        return Ok(());
    }

    // Ask user
    let prompt = format!("Are you sure to edit the profile data `{}`?", profile.name);
    let confirmed = dialog::confirm(&prompt, false).wrap_err("failed to show confirm dialog")?;

    // Do actions
    if confirmed {
        rt.block_on(async {
            profile
                .update_data(ProfileManager::get_instance().await)
                .await?;
            fs::write(&path, &contents)
                .await
                .wrap_err_with(|| format!("failed to write file `{}`", path.display()))
        })?;
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

pub fn ext_conf(target: ProfileGlobalTarget, no_reactivate: bool) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch target
    let target = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;
    let path = match &target {
        GlobalTargetFetchResult::Global => profile::GLOB_EXT_CONF_PATH.clone(),
        GlobalTargetFetchResult::Profile(profile) => profile.get_ext_conf_path(),
    };

    // Edit
    let contents = rt.block_on(async {
        if path.is_file() {
            tools::edit_file(&path)
                .await
                .wrap_err("failed to edit profile extend configurations")
        } else {
            tools::edit_contents(
                "",
                if target.is_global() {
                    "ext.yaml"
                } else {
                    "profile.ext.yaml"
                },
            )
            .await
            .wrap_err("failed to edit profile extend configurations")
        }
    })?;

    // Validate contents
    if contents.trim().is_empty() {
        error!("Profile extend configurations CANNOT be empty");
        return Ok(());
    }

    // Ask user
    let prompt = format!(
        "Are you sure to edit the profile extend configuration `{}`?",
        path.display()
    );
    let confirmed = dialog::confirm(&prompt, false).wrap_err("failed to show confirm dialog")?;

    // Do actions
    if confirmed {
        rt.block_on(async {
            fs::write(&path, &contents)
                .await
                .wrap_err_with(|| format!("failed to write file `{}`", path.display()))
        })?;
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

pub fn ext_scr(target: ProfileGlobalTarget, no_reactivate: bool) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch target
    let target = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;
    let path = match &target {
        GlobalTargetFetchResult::Global => profile::GLOB_EXT_SCR_PATH.clone(),
        GlobalTargetFetchResult::Profile(profile) => profile.get_ext_scr_path(),
    };

    // Edit
    let contents = rt.block_on(async {
        if path.is_file() {
            tools::edit_file(&path)
                .await
                .wrap_err("failed to edit profile extend scripts")
        } else {
            tools::edit_contents(
                "",
                if target.is_global() {
                    "ext.js"
                } else {
                    "profile.ext.js"
                },
            )
            .await
            .wrap_err("failed to edit profile extend scripts")
        }
    })?;

    // Ask user
    let prompt = format!(
        "Are you sure to edit the profile extend script `{}`?",
        path.display()
    );
    let confirmed = dialog::confirm(&prompt, false).wrap_err("failed to show confirm dialog")?;

    // Do actions
    if confirmed {
        rt.block_on(async {
            fs::write(&path, &contents)
                .await
                .wrap_err_with(|| format!("failed to write file `{}`", path.display()))
        })?;
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
