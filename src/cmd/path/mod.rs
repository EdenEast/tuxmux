use clap::{ArgMatches, Command};
use eyre::Result;

mod add;
mod list;
mod remove;

pub fn make_subcommand() -> Command {
    Command::new("path")
        .override_usage("tm path [COMMAND] [OPTIONS]")
        .about("Manage registered search paths")
        .disable_version_flag(true)
        .disable_colored_help(true)
        .subcommand(add::make_subcommand())
        .subcommand(list::make_subcommand())
        .subcommand(remove::make_subcommand())
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("add", sub_matches)) => add::execute(sub_matches),
        Some(("list", sub_matches)) => list::execute(sub_matches),
        Some(("remove", sub_matches)) => remove::execute(sub_matches),
        _ => {
            make_subcommand().print_long_help()?;
            Ok(())
        }
    }
}
