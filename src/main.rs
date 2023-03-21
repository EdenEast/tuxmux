#![allow(unused)]

use std::ffi::OsString;

use clap::{ArgMatches, Command};
use eyre::Result;
use tmgr::{cmd, data::Settings, finder::FinderOptions};

type ExecuteCmd = fn(&ArgMatches) -> Result<()>;

fn get_execute_cmd(name: &str) -> Option<(&str, ExecuteCmd)> {
    dbg!(name);
    match name {
        "attach" | "a" => Some(("attach", cmd::attach::execute)),
        "completions" => Some(("completions", cmd::completions::execute)),
        "config" | "c" => Some(("config", cmd::config::execute)),
        "jump" | "j" => Some(("jump", cmd::jump::execute)),
        "kill" | "k" => Some(("kill", cmd::kill::execute)),
        "list" | "ls" => Some(("list", cmd::list::execute)),
        "path" | "p" => Some(("path", cmd::path::execute)),
        "wcmd" | "w" => Some(("wcmd", cmd::wcmd::execute)),
        _ => None,
    }
}

fn execute_subcommand_if_exists<I, T>(name: &str, cmd: &Command, iter: I) -> Result<bool>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    if let Some((cmd_name, execute)) = get_execute_cmd(name) {
        let subcmd = cmd
            .get_subcommands()
            .find(|c| c.get_name() == name)
            .expect("Only valid names returned from get_execute_cmd");

        return execute(&subcmd.clone().get_matches_from(iter)).map(|_| true);
    }

    Ok(false)
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let first = args.next();
    let mut cmd = cmd::make_clap_command();

    dbg!(&args, &first);

    if let Some(first) = first {
        if first == "." {
            return cmd::attach::use_cwd();
        }

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

    if std::env::args().any(|a| matches!(a.as_str(), "-V" | "--version")) {
        print!("{}", cmd.render_version());
        return Ok(());
    }

    cmd::attach::execute(&cmd::attach::make_subcommand().get_matches())
}
