use eyre::{Context, OptionExt, Result};
use tokio::runtime::Runtime;

use crate::{
    cli::profile::{ProfileGlobalTarget, ProfileTarget},
    models::{profile, profile_manager::fetch::GlobalTargetFetchResult},
    utils::{log::log, tools},
};

pub fn info(target: ProfileTarget) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch profile
    let profile = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;

    // Render profile
    let contents = profile
        .render()
        .wrap_err("failed to render profile information")?;

    // Show
    rt.block_on(async {
        tools::view_contents(&contents, "info.yaml")
            .await
            .wrap_err("failed to show profile information")
    })
}

pub fn data(target: ProfileTarget) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Fetch profile
    let profile = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;

    // Check file
    let path = profile.get_data_path();
    if !path.is_file() {
        log!("Profile data not found. Please edit or update the profile.");
        return Ok(());
    }

    // Show
    rt.block_on(async {
        tools::view_file(&path)
            .await
            .wrap_err_with(|| format!("failed to show profile data `{}`", path.display()))
    })
}

pub fn ext_conf(target: ProfileGlobalTarget) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Get path
    let target = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;
    let path = match &target {
        GlobalTargetFetchResult::Global => profile::GLOB_EXT_CONF_PATH.clone(),
        GlobalTargetFetchResult::Profile(profile) => profile.get_ext_conf_path(),
    };

    // Check file
    if !path.is_file() {
        log!("Profile extend configurations not found. Please edit first.");
        return Ok(());
    }

    // Show
    rt.block_on(async {
        tools::view_file(&path).await.wrap_err_with(|| {
            format!(
                "failed to show profile extend configurations `{}`",
                path.display()
            )
        })
    })
}

pub fn ext_scr(target: ProfileGlobalTarget) -> Result<()> {
    // Create async runtime
    let rt = Runtime::new().wrap_err("failed to create async runtime")?;

    // Get path
    let target = rt
        .block_on(target.fetch_profile())?
        .ok_or_eyre("profile not found with given target")?;
    let path = match &target {
        GlobalTargetFetchResult::Global => profile::GLOB_EXT_SCR_PATH.clone(),
        GlobalTargetFetchResult::Profile(profile) => profile.get_ext_scr_path(),
    };

    // Check file
    if !path.is_file() {
        log!("Profile extend scripts not found. Please edit first.");
        return Ok(());
    }

    // Show
    rt.block_on(async {
        tools::view_file(&path)
            .await
            .wrap_err_with(|| format!("failed to show profile extend scripts `{}`", path.display()))
    })
}
