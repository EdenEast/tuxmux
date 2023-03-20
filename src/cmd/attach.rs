use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{data::Settings, finder::FinderOptions, tmux, util};
use clap::{value_parser, Arg, ArgMatches, Command};
use eyre::Result;
use rayon::prelude::*;

pub fn make_subcommand() -> Command {
    Command::new("attach")
        .about("Create or attach to a tmux session based on the path specified")
        .bin_name("tm attach")
        .visible_alias("a")
        .disable_version_flag(true)
        .disable_colored_help(true)
        .args(&[
            Arg::new("exist")
                .help("Attach to existing tmux session")
                .short('e')
                .long("exist")
                .action(clap::ArgAction::SetTrue),
            Arg::new("exact")
                .help("Use exact match search")
                .short('x')
                .long("exact")
                .action(clap::ArgAction::SetTrue),
            Arg::new("path")
                .help("Exact path to create or attach tmux session")
                .short('p')
                .long("path")
                .num_args(1)
                .value_parser(value_parser!(PathBuf)),
            Arg::new("query")
                .help("Query to search from")
                .required(false)
                .num_args(0..),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let settings = Settings::new()?;
    let query = matches
        .get_many::<String>("query")
        .map(|vs| vs.map(|s| s.as_str()).collect::<Vec<_>>().join(" "));

    let exact = matches.get_flag("exact");

    if matches.get_flag("exist") {
        let names = crate::tmux::session_names()?;
        let opts = FinderOptions {
            exact,
            query,
            height: settings.height,
            ..Default::default()
        };

        let selected = match names.len() {
            0 => return Ok(()),
            1 => names.into_iter().next(),
            _ => match settings.finder().execute(names.iter(), opts)? {
                Some(lines) => lines.into_iter().next(),
                None => return Ok(()),
            },
        };

        if let Some(selected) = selected {
            tmux::attach_session(selected.as_str())?;
        }

        return Ok(());
    }

    let paths = settings.list_paths();
    let selected = match get_selected(&paths, &query, matches, &settings) {
        Ok(Some(s)) => s,
        Ok(None) => return Ok(()),
        Err(e) => return Err(e),
    };

    execute_selected(&selected)
}

pub fn use_cwd() -> Result<()> {
    execute_selected(&std::env::current_dir()?)
}

fn execute_selected(selected: &Path) -> Result<()> {
    let name = util::format_name(selected.file_name().unwrap().to_str().unwrap());
    tmux::create_or_attach_session(&name, selected.to_str().unwrap())
}

fn get_selected(
    paths: &HashSet<String>,
    query: &Option<String>,
    matches: &ArgMatches,
    settings: &Settings,
) -> Result<Option<PathBuf>> {
    if let Some(path) = matches.get_one::<PathBuf>("path") {
        if path.as_path() == Path::new(".") {
            return Ok(Some(std::env::current_dir()?));
        }

        if !path.exists() {
            return Err(eyre::eyre!("Invalid path: '{}'", path.display()));
        }

        return Ok(Some(path.to_owned()));
    }

    if let Some(q) = query {
        let iter = paths.par_iter().filter(|v| v.contains(q));
        let count = iter.clone().count();
        if count == 1 {
            let l = iter.collect::<Vec<_>>();
            return Ok(Some(PathBuf::from_str(l[0])?));
        }
    }

    let exact = matches.get_flag("exact");

    let opts = FinderOptions {
        exact,
        query: query.clone(),
        height: settings.height,
        ..Default::default()
    };

    if let Some(lines) = settings.finder().execute(paths.iter(), opts)? {
        if let Some(first) = lines.into_iter().next() {
            return Ok(Some(PathBuf::from_str(first.as_str())?));
        }
    }

    Ok(None)
}
