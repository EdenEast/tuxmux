use crate::{data::Settings, fuzzy, tmux};
use clap::{Arg, ArgMatches, Command};
use eyre::Result;

pub fn make_subcommand() -> Command {
    Command::new("kill")
        .about("Kill a running tmux session")
        .alias("k")
        .disable_version_flag(true)
        .args(&[
            Arg::new("all")
                .help("Kill all sesssions")
                .short('a')
                .long("all")
                .action(clap::ArgAction::SetTrue),
            Arg::new("exact")
                .help("Use exact match search")
                .short('x')
                .long("exact")
                .action(clap::ArgAction::SetTrue),
            Arg::new("query")
                .help("Query to search from")
                .required(false)
                .num_args(0..),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<bool> {
    let settings = Settings::new()?;
    let query = matches
        .get_many::<String>("query")
        .map(|vs| vs.map(|s| s.as_str()).collect::<Vec<_>>().join(" "));

    let names = tmux::session_names()?;
    let selected = if matches.get_flag("all") {
        names
    } else {
        fuzzy::fuzzy_select_multi(
            names.iter().map(|a| a.as_str()),
            query.as_deref(),
            matches.get_flag("exact"),
            &settings,
        )
    };

    for sel in selected {
        tmux::kill_session(&sel)?;
        println!("Killed {}", sel);
    }

    Ok(true)
}
