use std::{
    fs::File,
    io::Write,
    path::Path,
    process::{self, ExitStatus},
};

use clap::Parser;
use miette::{IntoDiagnostic, Result};
use tuxmux::{
    cmd::{self, Run},
    config::Config,
    util,
};

const VALID_FIRST_OPTIONS: [&str; 15] = [
    "attach",
    "a",
    "config",
    "c",
    "completion",
    "jump",
    "j",
    "kill",
    "k",
    "list",
    "ls",
    "path",
    "p",
    "wcmd",
    "w",
];

const HELP_AND_VERSION_FLAGS: [&str; 4] = ["--help", "-h", "-V", "--version"];

fn editor(path: &Path) -> Result<ExitStatus> {
    process::Command::new(util::get_editor())
        .arg(path)
        .spawn()
        .into_diagnostic()?
        .wait()
        .into_diagnostic()
}

fn create_config_file(path: &Path) -> Option<File> {
    if !path.exists() {
        let parent = path
            .parent()
            .expect("config path contains a parent directory");
        std::fs::create_dir_all(parent).ok()?;
        File::create(path).ok()
    } else {
        None
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().collect::<Vec<_>>();

    env_logger::init();

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

    if cmd.edit {
        let path = util::get_config(&["config.kdl"]);
        if let Some(mut file) = create_config_file(&path) {
            file.write_all(cmd::DEFAULT_CONFIG.as_bytes())
                .into_diagnostic()?;
        }
        std::process::exit(editor(&path)?.code().unwrap_or(1));
    }

    if cmd.local {
        let path = util::get_local(&["config.kdl"]);
        create_config_file(&path);
        std::process::exit(editor(&path)?.code().unwrap_or(1));
    }

    match cmd.command {
        Some(cmd::Cmd::Attach(c)) => c.run(),
        Some(cmd::Cmd::Completion(c)) => c.run(),
        Some(cmd::Cmd::Jump(c)) => c.run(),
        Some(cmd::Cmd::Kill(c)) => c.run(),
        Some(cmd::Cmd::List(c)) => c.run(),
        Some(cmd::Cmd::Wcmd(c)) => c.run(),
        _ => Ok(()),
    }
}
