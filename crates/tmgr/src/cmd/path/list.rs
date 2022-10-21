use crate::data::{Location, Settings};
use clap::{Arg, ArgMatches, Command};
use eyre::Result;

pub fn make_subcommand() -> Command {
    Command::new("list")
        .about("List registered workspace and single paths")
        .disable_version_flag(true)
        .args(&[
            Arg::new("workspace")
                .help("Use path as a workspace path")
                .short('w')
                .long("workspace")
                .action(clap::ArgAction::SetTrue),
            Arg::new("single")
                .help("Use path as a workspace path")
                .short('s')
                .long("single")
                .action(clap::ArgAction::SetTrue),
            Arg::new("global")
                .help("Save to global $XDG_CONFIG_HOME instead of $XDG_DATA_HOME")
                .short('g')
                .long("global")
                .action(clap::ArgAction::SetTrue),
            Arg::new("local")
                .help("Save to global $XDG_CONFIG_HOME instead of $XDG_DATA_HOME")
                .short('l')
                .long("local")
                .action(clap::ArgAction::SetTrue),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<bool> {
    let settings = match (matches.get_flag("global"), matches.get_flag("local")) {
        (true, false) => Settings::from_location(Location::Global)?,
        (false, true) => Settings::from_location(Location::Local)?,
        _ => Settings::new()?,
    };

    if matches.get_flag("single") {
        settings.single_paths.iter().for_each(|s| println!("{}", s));
    }
    else if matches.get_flag("workspace") {
        settings.workspace_paths.iter().for_each(|s| println!("{}", s));
    }
    else {
        settings
            .single_paths
            .iter()
            .for_each(|s| println!("s {}", s));

        settings
            .workspace_paths
            .iter()
            .for_each(|s| println!("w {}", s));
    }

    Ok(true)
}
