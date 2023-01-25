use clap::{crate_description, crate_version, Command};

pub mod attach;
pub mod completions;
pub mod config;
pub mod jump;
pub mod kill;
pub mod list;
pub mod path;
pub mod wcmd;

const AFTER_HELP_MSG: &str = "\
By default if there is no command passed as the first argument the \
command 'attach' will be assumed. \
";

pub fn make_clap_command() -> Command {
    Command::new("tm")
        .bin_name("tm")
        .about(crate_description!())
        .after_help(AFTER_HELP_MSG)
        .version(crate_version!())
        .allow_external_subcommands(true)
        .allow_hyphen_values(true)
        .disable_help_subcommand(true)
        .subcommand(attach::make_subcommand())
        .subcommand(completions::make_subcommand())
        .subcommand(config::make_subcommand())
        .subcommand(jump::make_subcommand())
        .subcommand(kill::make_subcommand())
        .subcommand(list::make_subcommand())
        .subcommand(path::make_subcommand())
        .subcommand(wcmd::make_subcommand())
}
