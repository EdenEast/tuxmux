use std::{
    io::{BufWriter, Write},
    process::{self, Command, Stdio},
    str::FromStr,
};

use miette::{miette, IntoDiagnostic, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Global,
    Local,
}

#[derive(Debug, Default, Clone)]
pub struct FinderOptions {
    pub height: Option<usize>,
    // TODO: pass a string reference here
    pub query: Option<String>,
    pub exact: bool,
    pub multi: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum FinderChoice {
    #[default]
    Fzf,
    Skim,
}

pub const POSSIBLE_VALUES: &[&str] = &["fzf", "skim"];

impl FromStr for FinderChoice {
    type Err = miette::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "fzf" => Ok(FinderChoice::Fzf),
            "skim" => Ok(FinderChoice::Skim),
            _ => Err(miette!(
                "Unknown finder choice. Possible values: {:?}",
                POSSIBLE_VALUES
            )),
        }
    }
}

impl FinderChoice {
    pub fn execute<S, I>(&self, items: I, opts: FinderOptions) -> Result<Vec<String>>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let cmd = match self {
            FinderChoice::Fzf => "fzf",
            FinderChoice::Skim => "sk",
        };

        let mut command = Command::new(cmd);
        command.args([
            "--reverse",
            "--keep-right",
            "--exit-0",
            "--select-1",
            "--ansi",
        ]);

        if opts.exact {
            command.args(["--exact"]);
        }

        if opts.multi {
            command.args(["--multi"]);
        }

        if let Some(height) = opts.height {
            command.args(["--height", &format!("{}%", height)]);
        }

        if let Some(query) = opts.query.as_ref() {
            command.args(["--query", query]);
        }

        let mut child = match command.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
            Ok(x) => x,
            Err(_) => {
                let repo = match self {
                    Self::Fzf => "https://github.com/junegunn/fzf",
                    Self::Skim => "https://github.com/lotabout/skim",
                };
                eprintln!(
                    "tm was unable to call '{cmd}'. \
                Please make sure it's correctly installed. \
                Refer to {repo} for more info.",
                );
                std::process::exit(33)
            }
        };

        if let Some(stdin) = child.stdin.as_mut() {
            let mut writer = BufWriter::new(stdin);
            for i in items {
                writer.write_all(i.as_ref().as_bytes()).into_diagnostic()?;
                writer.write_all(b"\n").into_diagnostic()?;
            }
        }

        let out = child.wait_with_output().into_diagnostic()?;
        let text = match out.status.code() {
            Some(0) | Some(1) | Some(2) => {
                String::from_utf8(out.stdout).into_diagnostic()?
                // .context("Invalid utf8 received from finder")?
            }
            Some(130) => process::exit(130),
            _ => {
                let err = String::from_utf8(out.stderr)
                    .unwrap_or_else(|_| "<stderr contains invalid UTF-8>".to_owned());
                panic!("External command failed:\n {}", err)
            }
        };

        Ok(text.lines().map(ToOwned::to_owned).collect())
    }
}
