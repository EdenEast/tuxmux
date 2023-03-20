use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus, Stdio},
    str::FromStr,
};

use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
pub struct FinderOptions {
    pub height: Option<usize>,
    // TODO: pass a string reference here
    pub query: Option<String>,
    pub exact: bool,
    pub multi: bool,
}

pub struct FeedAndRead {
    lines: Vec<String>,
    status: ExitStatus,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FinderChoice {
    #[default]
    Fzf,
    Skim,
}

pub const POSSIBLE_VALUES: &[&str] = &["fzf", "skim"];

impl FromStr for FinderChoice {
    type Err = eyre::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "fzf" => Ok(FinderChoice::Fzf),
            "skim" => Ok(FinderChoice::Skim),
            _ => Err(eyre!(
                "Unknown finder choice. Possible values: {:?}",
                POSSIBLE_VALUES
            )),
        }
    }
}

impl FinderChoice {
    pub fn execute<S, I>(&self, items: I, opts: FinderOptions) -> Result<Option<Vec<String>>>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let cmd = match self {
            FinderChoice::Fzf => "fzf",
            FinderChoice::Skim => "sk",
        };

        let mut command = Command::new(cmd);
        command.args(&["--reverse", "--keep-right", "--exit-0", "--select-1"]);

        if opts.exact {
            command.args(&["--exact"]);
        }

        if opts.multi {
            command.args(&["--multi"]);
        }

        if let Some(height) = opts.height {
            command.args(&["--height", &format!("{}%", height)]);
        }

        if let Some(query) = opts.query {
            command.args(&["--print-query", "--query", &query]);
        }

        let FeedAndRead { lines, status, .. } = self.feed_and_read(items, &cmd, &mut command)?;

        match status.code() {
            Some(0 | 1) => Ok(Some(lines)),
            Some(130) => Ok(None), // Interupted with either CTRL-c or ESC
            Some(n) => Err(eyre!("{} process execited with status code: {}", cmd, n)),
            None => {
                if status.core_dumped() {
                    Err(eyre!("Core dumped :("))
                } else if let Some(sig) = status.signal() {
                    Err(eyre!("{} killed by signal: {}", cmd, sig))
                } else {
                    Err(eyre!("{} process exited with status: {:?}", cmd, status))
                }
            }
        }
    }

    fn feed_and_read<S, I>(
        &self,
        items: I,
        cmd: &str,
        command: &mut Command,
    ) -> std::io::Result<FeedAndRead>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let child = command.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn();

        let mut child = match child {
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

        let mut writer = BufWriter::new(child.stdin.take().unwrap());
        for i in items {
            writer.write_all(i.as_ref().as_bytes())?;
            writer.write_all(b"\n")?;
        }

        writer.flush()?;
        drop(writer);

        let reader = BufReader::new(child.stdout.take().unwrap());
        let mut lines = Vec::new();
        for line in reader.lines() {
            let line = line?;
            lines.push(line);
        }

        Ok(FeedAndRead {
            lines,
            status: child.wait()?,
        })
    }
}
