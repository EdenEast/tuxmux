use std::path::PathBuf;

use crate::data::{Location, PathKind, Settings};
use clap::{value_parser, Arg, ArgMatches, Command};
use eyre::Result;

pub fn make_subcommand() -> Command {
    Command::new("add")
        .about("Register a path to use when listing paths to attach.")
        .disable_version_flag(true)
        .args(&[
            Arg::new("workspace")
                .help("Use path as a workspace path")
                .short('w')
                .long("workspace")
                .action(clap::ArgAction::SetTrue),
            Arg::new("global")
                .help("Save to global $XDG_CONFIG_HOME instead of $XDG_DATA_HOME")
                .short('g')
                .long("global")
                .action(clap::ArgAction::SetTrue),
            Arg::new("path")
                .help("Optional paths to be added. Uses 'cwd' if not present")
                .required(false)
                .num_args(0..)
                .value_parser(value_parser!(PathBuf)),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<bool> {
    let cwd = std::env::current_dir()?;
    let paths = match &matches.get_many::<PathBuf>("path") {
        Some(vr) => vr.clone().map(|p| p.as_path()).collect(),
        None => vec![cwd.as_path()],
    };

    let location = if matches.get_flag("global") {
        Location::Global
    } else {
        Location::Local
    };

    let kind = if matches.get_flag("workspace") {
        PathKind::Workspace
    } else {
        PathKind::Single
    };

    let mut settings = Settings::from_location(location)?;
    for path in paths {
        let p = path.display().to_string();
        if settings.contains_path(&p) {
            println!("Path exists: {}", p);
        } else {
            settings.add_path(p, kind);
        }
    }

    settings.write(location)?;

    Ok(true)
}
