use itertools::{intersperse, Itertools};
use miette::{IntoDiagnostic, Result};
use std::{path::Path, process::Command};

use super::Multiplexer;

#[derive(Debug, Default, Clone, Copy)]
pub struct Zellij;

impl Multiplexer for Zellij {
    fn list_sessions(&self) -> Vec<String> {
        let result = match Command::new("zellij").args(&["list-sessions"]).output() {
            Ok(output) => output,
            Err(_) => return Vec::new(),
        };

        if !result.status.success() || result.stdout.is_empty() {
            return Vec::new();
        }

        String::from_utf8_lossy(&result.stdout)
            .lines()
            .map(|s| s.trim_end_matches(" (current)").to_string())
            .collect_vec()
    }

    fn session_exists(&self, name: &str) -> bool {
        self.list_sessions().iter().any(|x| x.as_str() == name)
    }

    fn create_session(&self, name: &str, path: &Path) -> Result<()> {
        dbg!(Command::new("zellij")
            .args(&[
                "--session",
                name,
                "options",
                "--default-cwd",
                path.to_str().unwrap_or("''"),
                "--attach-to-session",
                "false",
            ])
            .output()
            .into_diagnostic())?;
        Ok(())
    }

    fn attach_session(&self, name: &str) -> Result<()> {
        println!("attaching to session: {name}");
        dbg!(Command::new("zellij")
            .args(&["attach", name])
            .output()
            .into_diagnostic())?;
        Ok(())
    }

    fn kill_session(&self, name: &str) -> Result<()> {
        Command::new("zellij")
            .args(&["kill-session", name])
            .output()
            .into_diagnostic()?;
        Ok(())
    }

    fn send_command(&self, name: &str, command: &str) -> Result<()> {
        Command::new("zellij")
            .args(&["--session", name, "action", "write"])
            .args(command.as_bytes().iter().map(|b| b.to_string()))
            .arg("13") // 13 send ENTER
            .output()
            .into_diagnostic()?;
        Ok(())
    }
}
