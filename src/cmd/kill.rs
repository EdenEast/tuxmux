use crate::{data::Settings, finder::FinderOptions, tmux};
use clap::{Arg, ArgMatches, Command};
use eyre::Result;

pub fn make_subcommand() -> Command {
    Command::new("kill")
        .about("Kill a running tmux session")
        .bin_name("tm kill")
        .visible_alias("k")
        .disable_version_flag(true)
        .disable_colored_help(true)
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

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let settings = Settings::new()?;
    let query = matches
        .get_many::<String>("query")
        .map(|vs| vs.map(|s| s.as_str()).collect::<Vec<_>>().join(" "));

    let names = tmux::session_names()?;
    let selected = if matches.get_flag("all") {
        names
    } else {
        let opts = FinderOptions {
            query,
            multi: true,
            height: settings.height,
            ..Default::default()
        };
        let finder = settings.finder.unwrap_or_default();
        finder.execute(names.iter(), opts)?.unwrap_or_default()
    };

    for sel in selected {
        tmux::kill_session(&sel)?;
        println!("Killed {}", sel);
    }

    Ok(())
}
