use std::path::Path;

use crate::{tmux, util::intersperse};
use clap::{Arg, ArgMatches, Command};
use eyre::Result;

const LONG_WINDOW_HELP: &str = "\
Name of the window to execute the command from. \
This name window name can be taken from a path. \
In this case the basename will be used. This is \
useful with git worktrees and different branches.
";

const LONG_CMD_HELP: &str = "\
The command to be executed in the tmux window. \
Passing this after '--' will make sure that no \
option parsing is completed and the entire command \
is sent to the tmux window. This however does not \
have to be after '--'.
";

const EXAMPLE_AFTER_HELP: &str = "\
EXAMPLES:
  tm wcmd server cd backend
  tm w foo/bar/baz -- make test
";

pub fn make_subcommand() -> Command {
    Command::new("wcmd")
        .about("Send a command to a execute in a tmux window")
        .bin_name("tm wcmd")
        .visible_alias("w")
        .after_help(EXAMPLE_AFTER_HELP)
        .disable_version_flag(true)
        .disable_colored_help(true)
        .args(&[
            Arg::new("window")
                .help("Name of the window to execute the command from")
                .long_help(LONG_WINDOW_HELP)
                .required(true),
            Arg::new("cmd")
                .help("The command to be executed in the tmux window")
                .long_help(LONG_CMD_HELP)
                .required(false)
                .num_args(0..),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let window = matches
        .get_one::<String>("window")
        .expect("Window is required");

    let name = Path::new(window).file_name().unwrap().to_str().unwrap();
    let session_name = tmux::session_name();
    let target = format!("{}:{}", session_name.trim(), name);

    if !tmux::session_exists(&target) {
        tmux::create_window(name)?;
    }

    let cmd: String = intersperse(
        matches
            .get_many::<String>("cmd")
            .into_iter()
            .flatten()
            .map(|f| f.as_str()),
        " ",
    )
    .collect();

    tmux::send_keys(&target, &cmd)?;
    tmux::send_keys(&target, "C-m")?;

    Ok(())
}
