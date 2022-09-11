#![allow(unused)]
use clap::{crate_description, crate_version, Command};
use eyre::Result;

mod cmd;

const AFTER_HELP_MSG: &str = "\
By default if there is no command passed as the first argument the \
command 'attach' will be assumed. \
";

fn main() -> Result<()> {
    let matches = make_clap_command().get_matches();

    let exec_subcommand = match matches.subcommand() {
        Some(("add", sub_matches)) => cmd::add::execute(sub_matches)?,
        Some(("attach", sub_matches)) => cmd::attach::execute(sub_matches)?,
        Some(("config", sub_matches)) => cmd::config::execute(sub_matches)?,
        Some(("jump", sub_matches)) => cmd::jump::execute(sub_matches)?,
        Some(("kill", sub_matches)) => cmd::kill::execute(sub_matches)?,
        Some(("list", sub_matches)) => cmd::list::execute(sub_matches)?,
        Some(("remove", sub_matches)) => cmd::remove::execute(sub_matches)?,
        Some(("wcmd", sub_matches)) => cmd::wcmd::execute(sub_matches)?,
        _ => false,
    };

    if !exec_subcommand {
        cmd::attach::execute(&cmd::attach::make_subcommand().get_matches())?;
    }

    Ok(())
}

fn make_clap_command() -> Command<'static> {
    Command::new("tm")
        .bin_name("tm")
        .about(crate_description!())
        .after_help(AFTER_HELP_MSG)
        .version(crate_version!())
        .allow_external_subcommands(true)
        .allow_hyphen_values(true)
        .subcommand(cmd::add::make_subcommand())
        .subcommand(cmd::attach::make_subcommand())
        .subcommand(cmd::config::make_subcommand())
        .subcommand(cmd::jump::make_subcommand())
        .subcommand(cmd::kill::make_subcommand())
        .subcommand(cmd::list::make_subcommand())
        .subcommand(cmd::remove::make_subcommand())
        .subcommand(cmd::wcmd::make_subcommand())
}