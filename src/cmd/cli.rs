use std::path::PathBuf;

use clap::{crate_description, crate_version, Subcommand};
use clap::{Args, Parser};

pub const CONFIG_OPTIONS: [&str; 3] = ["depth", "height", "finder"];

const ARG_AFTER_HELP_MSG: &str = "\
By default if there is no command passed as the first argument the \
command 'attach' will be assumed. \
";

const JUMP_LONG_ABOUT: &str = "\
Store a list of paths and jump to that index. This is useful for keybindings \
where you set keybindingd to jump to index 1, 2, 3, ... and tm will check \
the list of stored paths and use that to jump to that tmux session.

By default if no options are passed then the cwd is added to the jump list \
";

const WCMD_LONG_WINDOW_HELP: &str = "\
Name of the window to execute the command from. \
This name window name can be taken from a path. \
In this case the basename will be used. This is \
useful with git worktrees and different branches.
";

const WCMD_LONG_CMD_HELP: &str = "\
The command to be executed in the tmux window. \
Passing this after '--' will make sure that no \
option parsing is completed and the entire command \
is sent to the tmux window. This however does not \
have to be after '--'.
";

const WCMD_EXAMPLE_AFTER_HELP: &str = "\
EXAMPLES:
  tm wcmd server cd backend
  tm w foo/bar/baz -- make test
";

#[derive(Debug, Parser)]
#[command(
    name = "tm",
    bin_name("tm"),
    after_help = ARG_AFTER_HELP_MSG,
    about = crate_description!(),
    version = crate_version!(),
    disable_help_subcommand = true,
    allow_external_subcommands = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Attach(Attach),
    Jump(Jump),
    Kill(Kill),
    List(List),
    Wcmd(Wcmd),
}

/// Create or attach to a tmux session based on the path specified
#[derive(Debug, Args)]
#[command(
    visible_alias("a"),
    bin_name("tm-attach"),
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct Attach {
    /// Attach to existing session
    #[arg(short, long, default_value_t = false)]
    pub exists: bool,

    /// Use exact match search instead of fuzzy
    #[arg(short = 'x', long, default_value_t = false)]
    pub exact: bool,

    /// Exact path to either attach to existing session or create a new one if
    /// none exist
    #[arg(short, long, default_value = None)]
    pub path: Option<PathBuf>,

    /// Query to search from. If there is only one result that result will be
    /// automatically selected. If there are multiple results then a search
    /// field will be presented.
    #[arg(default_value = None)]
    pub query: Option<Vec<String>>,
}

/// Get or set configuration options for tm
#[derive(Debug, Args)]
#[command(
    visible_alias("c"),
    bin_name("tm-config"),
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct Config {
    /// Name of configuration option
    #[arg(value_parser = CONFIG_OPTIONS)]
    pub name: Option<String>,

    /// Value of the configuration option defined by name
    pub value: Option<String>,

    /// Save to global $XDG_CONFIG_HOME instead of $XDG_DATA_HOME
    #[arg(short, long, default_value_t = false)]
    pub global: bool,

    /// Open config file in '$EDITOR'
    #[arg(short, long, default_value_t = false)]
    pub edit: bool,

    /// List all config options and values
    #[arg(short, long, default_value_t = false)]
    pub list: bool,
}

/// Store paths and later jump to them by index
#[derive(Debug, Args)]
#[command(
    visible_alias("j"),
    bin_name("tm-jump"),
    disable_colored_help(true),
    disable_version_flag(true),
    long_about = JUMP_LONG_ABOUT,
)]
pub struct Jump {
    /// Open jump list file in "$EDITOR"
    #[arg(short, long, default_value_t = false)]
    pub edit: bool,

    /// List jump list
    #[arg(short, long, default_value_t = false)]
    pub list: bool,

    /// Jump to index in jump list. Index is 1 based
    #[arg(short, long)]
    pub index: Option<usize>,

    /// Add path to jump list
    #[arg(short, long, default_value = None)]
    pub path: Option<PathBuf>,
}

/// Kill a running tmux session
#[derive(Debug, Args)]
#[command(
    visible_alias("k"),
    bin_name("tm-kill"),
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct Kill {
    /// Kill all sessions
    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    /// Use exact match search instead of fuzzy
    #[arg(short = 'x', long, default_value_t = false)]
    pub exact: bool,

    /// Query to search from. If there is only one result that result will be
    /// automatically selected. If there are multiple results then a search
    /// field will be presented.
    #[arg(default_value = None)]
    pub query: Option<Vec<String>>,
}

/// List current sessions
#[derive(Debug, Args)]
#[command(
    visible_alias("ls"),
    bin_name("tm-list"),
    disable_colored_help(true),
    disable_version_flag(true)
)]
pub struct List {}

/// Send a command to a execute in a tmux window
#[derive(Debug, Args)]
#[command(
    visible_alias("w"),
    bin_name("tm-wcmd"),
    disable_colored_help(true),
    disable_version_flag(true),
    after_help = WCMD_EXAMPLE_AFTER_HELP,
)]
pub struct Wcmd {
    /// Name of the window to execute the command from
    #[arg(long_help = WCMD_LONG_WINDOW_HELP)]
    pub window: String,

    /// The command to be executed in the tmux window
    #[arg(long_help = WCMD_LONG_CMD_HELP)]
    pub cmds: Vec<String>,
}

// vim: textwidth=80
