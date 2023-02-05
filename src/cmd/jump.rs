use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{data::Jumplist, tmux};
use clap::{value_parser, Arg, ArgMatches, Command};
use eyre::Result;

const LONG_ABOUT: &str = "\
Store a list of paths and jump to that index. This is useful for keybindings \
where you set keybindingd to jump to index 1, 2, 3, ... and tm will check \
the list of stored paths and use that to jump to that tmux session.

By default if no options are passed then the cwd is added to the jump list \
";

pub fn make_subcommand() -> Command {
    Command::new("jump")
        .about("Store paths and later jump to them by index")
        .bin_name("tm jump")
        .visible_alias("j")
        .long_about(LONG_ABOUT)
        .disable_version_flag(true)
        .disable_colored_help(true)
        .args(&[
            Arg::new("edit")
                .help("Open jump list file in \"$EDITOR\"")
                .short('e')
                .long("edit")
                .action(clap::ArgAction::SetTrue),
            Arg::new("list")
                .help("List jump list")
                .short('l')
                .long("list")
                .action(clap::ArgAction::SetTrue),
            Arg::new("index")
                .help("Jump to index in jump list. Index is 1 based")
                .short('i')
                .long("index")
                .num_args(1)
                .value_parser(value_parser!(usize)),
            Arg::new("path")
                .help("Add path to jump list")
                .short('o')
                .long("path")
                .num_args(1)
                .value_parser(value_parser!(PathBuf)),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    if matches.get_flag("edit") {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_owned());
        std::process::Command::new(editor)
            .arg(Jumplist::path())
            .status()?;

        return Ok(());
    }

    let mut list = Jumplist::new()?;

    if matches.get_flag("list") {
        for (i, elem) in list.0.iter().enumerate() {
            println!("{}: {}", i + 1, elem)
        }

        return Ok(());
    }

    if let Some(index) = matches.get_one::<usize>("index") {
        if let Some(sel) = list.get((*index).saturating_sub(1)) {
            let name = Path::new(sel).file_name().unwrap().to_str().unwrap();
            tmux::create_or_attach_session(name, sel)?;
        }

        return Ok(());
    }

    let path = match matches.get_one::<PathBuf>("path") {
        Some(path) => path.clone(),
        None => std::env::current_dir()?,
    };

    if !path.exists() {
        // TODO: Add logging with different types of sevarity
        return Err(eyre::eyre!("Invalid path: {}", path.display()));
    }

    list.add(path.canonicalize()?.display().to_string());
    list.write()?;

    Ok(())
}