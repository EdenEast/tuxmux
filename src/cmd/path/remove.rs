use crate::{
    data::{Location, PathKind, Settings},
    finder::FinderOptions,
};
use clap::{Arg, ArgMatches, Command};
use eyre::Result;

pub fn make_subcommand() -> Command {
    Command::new("remove")
        .about("Remove registered path from tm")
        .bin_name("tm path remove")
        .disable_version_flag(true)
        .disable_colored_help(true)
        .args(&[
            Arg::new("workspace")
                .help("Remove only workspace paths")
                .short('w')
                .long("workspace")
                .action(clap::ArgAction::SetTrue),
            Arg::new("global")
                .help("Remove from global config")
                .short('g')
                .long("global")
                .action(clap::ArgAction::SetTrue),
            Arg::new("exact")
                .help("Use exact match search")
                .short('x')
                .long("exact")
                .action(clap::ArgAction::SetTrue),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let location = if matches.get_flag("global") {
        Location::Global
    } else {
        Location::Local
    };

    let mut settings = Settings::from_location(location)?;
    let iter: Vec<String> = settings
        .workspace_paths
        .iter()
        .cloned()
        .map(|mut s| {
            s.insert_str(0, "w| ");
            s
        })
        .chain(settings.single_paths.iter().cloned().map(|mut s| {
            s.insert_str(0, "s| ");
            s
        }))
        .collect();

    let opts = FinderOptions {
        multi: true,
        height: settings.height,
        ..Default::default()
    };

    let selected = match settings.finder().execute(iter.iter(), opts)? {
        Some(selected) => selected,
        None => {
            return Ok(());
        }
    };

    for sel in selected {
        let (k, v) = sel.split_at(3);
        let kind = if k.starts_with('s') {
            PathKind::Single
        } else {
            PathKind::Workspace
        };
        match kind {
            PathKind::Single => settings.single_paths.remove(v),
            PathKind::Workspace => settings.workspace_paths.remove(v),
        };
    }

    settings.write(location)?;

    Ok(())
}
