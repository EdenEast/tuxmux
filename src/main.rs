use clap::Parser;
use miette::Result;
use tuxmux::{
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
                let config = Config::load()?;
                return cmd::Attach::default().use_cwd(&config);
            }

            let starts_with_long = first.starts_with("--");
            let contains_help_or_version = HELP_AND_VERSION_FLAGS.iter().any(|v| *v == first);
            if !contains_help_or_version
                && !starts_with_long
                && !VALID_FIRST_OPTIONS.iter().any(|v| *v == first)
            {
                args.insert(1, "attach".to_owned());
            }
        }
        None => args.push("attach".to_owned()),
    }

    let cmd = cmd::Cli::parse_from(args);
    if cmd.default_config {
        println!("{}", cmd::DEFAULT_CONFIG);
        return Ok(());
    }

    match cmd.command {
        Some(cmd::Cmd::Attach(c)) => c.run(),
        Some(cmd::Cmd::Jump(c)) => c.run(),
        Some(cmd::Cmd::Kill(c)) => c.run(),
        Some(cmd::Cmd::List(c)) => c.run(),
        Some(cmd::Cmd::Wcmd(c)) => c.run(),
        _ => Ok(()),
    }
}
