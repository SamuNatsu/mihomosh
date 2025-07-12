mod arguments;
mod commands;
mod includes;
mod models;
mod utils;

use std::io;

use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::{
    arguments::{
        Args,
        config::ConfigArgs,
        connection::ConnectionArgs,
        control::ControlArgs,
        inspect::InspectArgs,
        profile::{ProfileArgs, ProfileEditArgs, ProfileViewArgs},
    },
    commands::{
        config, connection, control, inspect, profile, profile_edit, profile_global, profile_view,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    match Args::parse() {
        Args::Config(args) => match args {
            ConfigArgs::View { viewer } => config::view(viewer)?,
            ConfigArgs::Edit { editor } => config::edit(editor)?,
            ConfigArgs::Reset => config::reset()?,
        },
        Args::Profile(args) => match args {
            ProfileArgs::Update { uuid_or_name } => profile::update(uuid_or_name).await?,
            ProfileArgs::Activate { uuid_or_name } => (),
            ProfileArgs::Create { editor } => profile::create(editor)?,
            ProfileArgs::Delete { uuid_or_name } => profile::delete(uuid_or_name)?,
            ProfileArgs::List => profile::list()?,
            ProfileArgs::View(args) => match args {
                ProfileViewArgs::Info {
                    uuid_or_name,
                    viewer,
                } => profile_view::view_info(uuid_or_name, viewer)?,
                ProfileViewArgs::File {
                    uuid_or_name,
                    viewer,
                } => profile_view::view_file(uuid_or_name, viewer)?,
                ProfileViewArgs::ExtendConfig {
                    uuid_or_name,
                    viewer,
                } => profile_view::view_ext_conf(uuid_or_name, viewer)?,
                ProfileViewArgs::ExtendScript {
                    uuid_or_name,
                    viewer,
                } => profile_view::view_ext_script(uuid_or_name, viewer)?,
            },
            ProfileArgs::Edit(args) => match args {
                ProfileEditArgs::Info {
                    uuid_or_name,
                    editor,
                } => profile_edit::edit_info(uuid_or_name, editor)?,
                ProfileEditArgs::File {
                    uuid_or_name,
                    editor,
                } => profile_edit::edit_file(uuid_or_name, editor)?,
                ProfileEditArgs::ExtendConfig {
                    uuid_or_name,
                    editor,
                } => profile_edit::edit_ext_conf(uuid_or_name, editor)?,
                ProfileEditArgs::ExtendScript {
                    uuid_or_name,
                    editor,
                } => profile_edit::edit_ext_script(uuid_or_name, editor)?,
            },
            ProfileArgs::ViewGlobalExtendConfig { viewer } => {
                profile_global::view_ext_conf(viewer)?
            }
            ProfileArgs::ViewGlobalExtendScript { viewer } => {
                profile_global::view_ext_script(viewer)?
            }
            ProfileArgs::EditGlobalExtendConfig { editor } => {
                profile_global::edit_ext_conf(editor)?
            }
            ProfileArgs::EditGlobalExtendScript { editor } => {
                profile_global::edit_ext_script(editor)?
            }
        },
        Args::Connection(args) => match args {
            ConnectionArgs::View => connection::view().await?,
            ConnectionArgs::Close {
                r#type,
                host,
                process,
                source,
                destination,
                chain,
                rule,
            } => connection::close(r#type, host, process, source, destination, chain, rule).await?,
        },
        Args::Inspect(args) => match args {
            InspectArgs::Log => inspect::log().await?,
            InspectArgs::Traffic => inspect::traffic().await?,
            InspectArgs::Memory => inspect::memory().await?,
            InspectArgs::Version => inspect::version().await?,
        },
        Args::Control(args) => match args {
            ControlArgs::FlushCache => control::flush_cache().await?,
            ControlArgs::UpdateUi => control::update_ui().await?,
            ControlArgs::UpdateGeo => control::update_geo().await?,
            ControlArgs::Restart => control::restart().await?,
        },
        Args::ShellCompletion { shell } => {
            let mut cmd = Args::command();
            let bin_name = cmd.get_name().to_owned();

            clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
        }
        _ => (),
    }

    Ok(())
}
