use std::{collections::HashSet, path::PathBuf, str::FromStr};

use crate::{data::Settings, tmux};
use clap::{value_parser, Arg, ArgMatches, Command};
use eyre::Result;
use rayon::prelude::*;

pub fn make_subcommand() -> Command<'static> {
    Command::new("attach")
        .about("Create or attach to a tmux session based on the path specified")
        .alias("a")
        .disable_version_flag(true)
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
                .takes_value(true)
                .value_parser(value_parser!(PathBuf)),
            Arg::new("query")
                .help("Query to search from")
                .required(false)
                .multiple_values(true),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<bool> {
    let query = matches
        .get_many::<String>("query")
        .map(|vs| vs.map(|s| s.as_str()).collect::<Vec<_>>().join(" "));

    let exact = matches.get_flag("exact");
    if matches.get_flag("exist") {
        let names = crate::tmux::session_names()?;

        let selected = match names.len() {
            0 => return Ok(true),
            1 => names[0].clone(),
            _ => match crate::fuzzy::fuzzy_select_one(
                names.iter().map(|a| a.as_str()),
                query.as_deref(),
                exact,
            ) {
                Some(index) => index,
                None => return Ok(true),
            },
        };

        tmux::attach_session(&selected)?;

        return Ok(true);
    }

    let settings = Settings::new()?;
    let paths = settings.list_paths();
    let selected = match get_selected(&paths, &query, matches) {
        Ok(Some(s)) => s,
        Ok(None) => return Ok(true),
        Err(e) => return Err(e),
    };

    let name = selected.as_path().file_name().unwrap().to_str().unwrap();

    if !tmux::session_exists(name) {
        tmux::create_session(name, selected.to_str().unwrap())?;
    }

    tmux::attach_session(name)?;

    Ok(true)
}

fn get_selected(
    paths: &HashSet<String>,
    query: &Option<String>,
    matches: &ArgMatches,
) -> Result<Option<PathBuf>> {
    if let Some(path) = matches.get_one::<PathBuf>("path") {
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
    match crate::fuzzy::fuzzy_select_one(paths.iter().map(|a| a.as_str()), query.as_deref(), exact)
    {
        Some(sel) => Ok(Some(PathBuf::from_str(&sel)?)),
        None => Ok(None),
    }
}
