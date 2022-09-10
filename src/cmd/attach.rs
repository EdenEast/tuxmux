use std::{path::PathBuf, str::FromStr};

use clap::{Arg, ArgMatches, Command};
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
            Arg::new("path")
                .help("Exact path to create or attach tmux session")
                .short('p')
                .long("path")
                .takes_value(true),
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

    if matches.get_flag("exist") {
        let names = tmgr::tmux::get_sessions()?;

        let selected = match names.len() {
            0 => return Ok(true),
            1 => names[0].clone(),
            _ => match tmgr::fuzzy::fuzzy_select_one(
                names.iter().map(|a| a.as_str()),
                query.as_deref(),
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
    let selected =
        match tmgr::fuzzy::fuzzy_select_one(paths.iter().map(|a| a.as_str()), query.as_deref()) {
            Some(sel) => sel,
            None => return Ok(true),
        };

    let path = PathBuf::from_str(&selected)?;
    let name = path.as_path().file_name().unwrap().to_str().unwrap();

    if !tmux::session_exists(name) {
        tmux::create_session(name, &selected)?;
    }

    tmux::attach_session(name)?;

    Ok(true)
}
