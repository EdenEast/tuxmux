use clap::Parser;
use miette::Result;
use tmgr::{
    cmd::{self, Run},
    config::Config,
};

const VALID_FIRST_OPTIONS: [&str; 14] = [
    "attach", "a", "config", "c", "jump", "j", "kill", "k", "list", "ls", "path", "p", "wcmd", "w",
];

const HELP_AND_VERSION_FLAGS: [&str; 4] = ["--help", "-h", "-V", "--version"];

fn main() -> Result<()> {
    let mut args = std::env::args().collect::<Vec<_>>();

    match args.get(1) {
        Some(first) => {
            if first == "." {
                return cmd::use_cwd();
            }

            let contains_help_or_version = HELP_AND_VERSION_FLAGS.iter().any(|v| *v == first);
            if !contains_help_or_version && !VALID_FIRST_OPTIONS.iter().any(|v| *v == first) {
                args.insert(1, "attach".to_owned());
            }
        }
        None => args.push("attach".to_owned()),
    }

    match cmd::Cli::parse_from(args).command {
        cmd::Cmd::Attach(c) => c.run(),
        cmd::Cmd::Jump(c) => c.run(),
        cmd::Cmd::Kill(c) => c.run(),
        cmd::Cmd::List(c) => c.run(),
        cmd::Cmd::Wcmd(c) => c.run(),
    }
}
