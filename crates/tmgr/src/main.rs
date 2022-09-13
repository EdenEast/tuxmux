#![allow(unused)]
use clap::{builder::PossibleValuesParser, crate_description, crate_version, Arg, Command};
use clap_complete::Shell;
use eyre::Result;
use tmgr::cmd;

fn main() -> Result<()> {
    let matches = cmd::make_clap_command().get_matches();

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
