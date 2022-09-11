use std::{path::PathBuf, str::FromStr};

use clap::{value_parser, Arg, ArgMatches, Command};
use eyre::Result;
use tmgr::{data::Settings, tmux};

pub fn make_subcommand() -> Command<'static> {
    Command::new("attach")
        .about("Create or attach to a tmux session based on the path specified")
        .alias("a")
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
        let names = tmgr::tmux::session_names()?;

        let selected = match names.len() {
            0 => return Ok(true),
            1 => names[0].clone(),
            _ => match tmgr::fuzzy::fuzzy_select_one(
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
    let selected = if let Some(path) = matches.get_one::<PathBuf>("path") {
        if !path.exists() {
            return Err(eyre::eyre!("Invalid path: '{}'", path.display()));
        }
        path.to_owned()
    } else {
        match tmgr::fuzzy::fuzzy_select_one(
            paths.iter().map(|a| a.as_str()),
            query.as_deref(),
            exact,
        ) {
            Some(sel) => PathBuf::from_str(&sel)?,
            None => return Ok(true),
        }
    };

    let name = selected.as_path().file_name().unwrap().to_str().unwrap();

    if !tmux::session_exists(name) {
        tmux::create_session(name, selected.to_str().unwrap())?;
    }

    tmux::attach_session(name)?;

    Ok(true)
}
