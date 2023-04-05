use clap::Parser;
use eyre::Result;
use tmgr::{
    cli::Cli,
    cmd::{self, ExecuteableCmd},
};

mod cli;

const VALID_FIRST_OPTIONS: [&str; 14] = [
    "attach", "a", "config", "c", "jump", "j", "kill", "k", "list", "ls", "path", "p", "wcmd", "w",
];

const HELP_AND_VERSION_FLAGS: [&str; 4] = ["--help", "-h", "-V", "--version"];

fn main() -> Result<()> {
    let mut args = std::env::args().collect::<Vec<_>>();

    match args.get(1) {
        Some(first) => {
            if first == "." {
                return cmd::attach::use_cwd();
            }

            let contains_help_or_version = HELP_AND_VERSION_FLAGS.iter().any(|v| *v == first);
            if !contains_help_or_version && !VALID_FIRST_OPTIONS.iter().any(|v| *v == first) {
                args.insert(1, "attach".to_owned());
            }
        }
        None => args.push("attach".to_owned()),
    }

    match Cli::parse_from(args).command {
        tmgr::cli::Cmd::Attach(c) => c.execute(),
        tmgr::cli::Cmd::Config(c) => c.execute(),
        tmgr::cli::Cmd::Jump(c) => c.execute(),
        tmgr::cli::Cmd::Kill(c) => c.execute(),
        tmgr::cli::Cmd::List(c) => c.execute(),
        tmgr::cli::Cmd::Path(c) => c.execute(),
        tmgr::cli::Cmd::Wcmd(c) => c.execute(),
    }
}
