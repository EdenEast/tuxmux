#![allow(unused)]
use clap::{builder::PossibleValuesParser, crate_description, crate_version, Arg, Command};
use eyre::Result;
use tmgr::cmd;

fn main() -> Result<()> {
    match cmd::make_clap_command().try_get_matches() {
        Ok(matches) => match matches.subcommand() {
            Some(("attach", sub_matches)) => cmd::attach::execute(sub_matches)?,
            Some(("config", sub_matches)) => cmd::config::execute(sub_matches)?,
            Some(("jump", sub_matches)) => cmd::jump::execute(sub_matches)?,
            Some(("kill", sub_matches)) => cmd::kill::execute(sub_matches)?,
            Some(("list", sub_matches)) => cmd::list::execute(sub_matches)?,
            Some(("path", sub_matches)) => cmd::path::execute(sub_matches)?,
            Some(("wcmd", sub_matches)) => cmd::wcmd::execute(sub_matches)?,
            _ => unreachable!("Forgot to add new command main match statement"),
        },
        Err(e) => {
            cmd::attach::execute(&cmd::attach::make_subcommand().get_matches())?;
        }
    };

    Ok(())
}
