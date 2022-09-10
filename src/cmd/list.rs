use clap::{Arg, ArgMatches, Command};
use eyre::Result;
use tmgr::data::Settings;

pub fn make_subcommand() -> Command<'static> {
    Command::new("list")
        .about("List workspace and single paths registered to tm")
        .alias("l")
        .args(&[
            Arg::new("single")
                .help("Show only single paths")
                .short('s')
                .long("single")
                .action(clap::ArgAction::SetTrue),
            Arg::new("workspace")
                .help("Show only workspace paths")
                .short('w')
                .long("workspace")
                .action(clap::ArgAction::SetTrue),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<bool> {
    let settings = Settings::new()?;
    let single_iter = settings.single_paths.iter().cloned().map(|mut s| {
        s.insert_str(0, "s| ");
        s
    });

    let workspace_iter = settings.workspace_paths.iter().cloned().map(|mut s| {
        s.insert_str(0, "w| ");
        s
    });

    let (use_single, use_workspace) =
        match (matches.get_flag("single"), matches.get_flag("workspace")) {
            (true, false) => (true, false),
            (false, true) => (false, true),
            _ => (true, true),
        };

    if use_single {
        for v in single_iter {
            println!("{}", v);
        }
    }

    if use_workspace {
        for v in workspace_iter {
            println!("{}", v);
        }
    }

    Ok(true)
}
