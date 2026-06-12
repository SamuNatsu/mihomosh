pub mod edit;
pub mod misc;
pub mod show;

use eyre::Result;

use crate::cli::profile::{
    ProfileCommand, ProfileDataCommand, ProfileExtensionCommand, ProfileExtensionConfigCommand,
    ProfileExtensionScriptCommand,
};

pub fn handle(cmd: ProfileCommand) -> Result<()> {
    match cmd {
        ProfileCommand::List => misc::list(),
        ProfileCommand::Show { target } => show::info(target),
        ProfileCommand::Create => misc::create(),
        ProfileCommand::Edit { target } => edit::info(target),
        ProfileCommand::Delete { target } => misc::delete(target),
        ProfileCommand::Update {
            target,
            no_reactivate,
        } => misc::update(target, no_reactivate),
        ProfileCommand::Activate { .. } => todo!(),
        ProfileCommand::Reactivate => todo!(),
        ProfileCommand::Data(cmd) => match cmd {
            ProfileDataCommand::Show { target } => show::data(target),
            ProfileDataCommand::Edit {
                target,
                no_reactivate,
            } => edit::data(target, no_reactivate),
        },
        ProfileCommand::Extension(cmd) => match cmd {
            ProfileExtensionCommand::Config(cmd) => match cmd {
                ProfileExtensionConfigCommand::Show { target } => show::ext_conf(target),
                ProfileExtensionConfigCommand::Edit {
                    target,
                    no_reactivate,
                } => edit::ext_conf(target, no_reactivate),
            },
            ProfileExtensionCommand::Script(cmd) => match cmd {
                ProfileExtensionScriptCommand::Show { target } => show::ext_scr(target),
                ProfileExtensionScriptCommand::Edit {
                    target,
                    no_reactivate,
                } => edit::ext_scr(target, no_reactivate),
            },
        },
    }
}
