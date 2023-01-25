#![allow(unused)]

use std::ffi::OsString;

use clap::{ArgMatches, Command};
use eyre::Result;
use tmgr::cmd;

type ExecuteCmd = fn(&ArgMatches) -> Result<()>;

fn get_execute_cmd(name: &str) -> Option<ExecuteCmd> {
    match name {
        "attach" => Some(cmd::attach::execute),
        "completions" => Some(cmd::completions::execute),
        "config" => Some(cmd::config::execute),
        "jump" => Some(cmd::jump::execute),
        "kill" => Some(cmd::kill::execute),
        "list" => Some(cmd::list::execute),
        "path" => Some(cmd::path::execute),
        "wcmd" => Some(cmd::wcmd::execute),
        _ => None,
    }
}

fn execute_subcommand_if_exists<I, T>(name: &str, cmd: &Command, iter: I) -> Result<bool>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    if let Some(sub) = cmd.get_subcommands().find(|c| c.get_name() == name) {
        if let Some(execute) = get_execute_cmd(name) {
            let matches = sub.clone().get_matches_from(iter);
            execute(&matches)?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let first = args.next();
    let mut cmd = cmd::make_clap_command();

    if let Some(first) = first {
        if execute_subcommand_if_exists(&first, &cmd, std::env::args().take(1).chain(args))? {
            return Ok(());
        }
    }

    if let Some(help_arg) = std::env::args().find(|a| matches!(a.as_str(), "-h" | "--help")) {
        if help_arg == "-h" {
            cmd.print_help();
        } else {
            cmd.print_long_help();
        }
        return Ok(());
    }

    if std::env::args().any(|a| matches!(a.as_str(), "-V" | "--version")){
        print!("{}", cmd.render_version());
            return Ok(());
    }

    cmd::attach::execute(&cmd::attach::make_subcommand().get_matches())
}
