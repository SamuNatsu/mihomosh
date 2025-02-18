mod arg;
mod cmd;
mod data;
mod utils;

use anyhow::Result;
use arg::{
    config::Command as ConfigCommand, ctrl::Command as CtrlCommand,
    profile::Command as ProfileCommand, show::Command as ShowCommand,
    status::Command as StatusCommand, Args,
};
use clap::Parser;
use cmd::{config, ctrl, profile, show, status, test};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse commands
    let args = Args::parse();
    match args {
        Args::Config(cmd) => match cmd {
            ConfigCommand::Edit { editor } => config::edit(editor)?,
            ConfigCommand::Reset => config::reset()?,
            ConfigCommand::View => config::view()?,
        },
        Args::Ctrl(cmd) => match cmd {
            CtrlCommand::UpdateGeo => ctrl::update_geo().await?,
            CtrlCommand::UpdateGroup => ctrl::update_group().await?,
            CtrlCommand::Restart => ctrl::restart().await?,
        },
        Args::Profile(cmd) => match cmd {
            ProfileCommand::Activate { uuid_or_name } => profile::activate(uuid_or_name).await?,
            ProfileCommand::Delete { uuid_or_name } => profile::delete(uuid_or_name)?,
            ProfileCommand::EditConfigs { uuid_or_name } => profile::edit_conf(uuid_or_name)?,
            ProfileCommand::EditData { uuid_or_name } => profile::edit_data(uuid_or_name)?,
            ProfileCommand::List => profile::list()?,
            ProfileCommand::New => profile::new()?,
            ProfileCommand::Update { uuid_or_name } => profile::update(uuid_or_name).await?,
            ProfileCommand::ViewConfigs { uuid_or_name } => profile::view_conf(uuid_or_name)?,
            ProfileCommand::ViewData { uuid_or_name } => profile::view_data(uuid_or_name)?,
            ProfileCommand::ViewRules { uuid_or_name } => profile::view_rules(uuid_or_name)?,
        },
        Args::Show(cmd) => match cmd {
            ShowCommand::Profile => show::profile()?,
            ShowCommand::Rules => show::rules()?,
        },
        Args::Status(cmd) => match cmd {
            StatusCommand::Configs => status::configs().await?,
            StatusCommand::Connections => status::connections().await?,
            StatusCommand::Groups => status::groups().await?,
            StatusCommand::Version => status::version().await?,
        },
        Args::Test { url } => test::test(url).await?,
    }

    // Success
    Ok(())
}
