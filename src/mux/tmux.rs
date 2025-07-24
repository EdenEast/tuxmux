use std::path::Path;
use std::process;

use itertools::Itertools;
use log::debug;
use log::log_enabled;
use log::Level;

use super::Mux;
use super::OutputExtension;

#[derive(Clone, Debug)]
pub struct Tmux {
    socket_name: String,
}

impl Default for Tmux {
    fn default() -> Self {
        let socket_name = std::env::var("TMS_TMUX_SOCKET")
            .ok()
            .unwrap_or("default".to_string());
        Self { socket_name }
    }
}

impl Tmux {
    fn execute_tmux_command(&self, args: &[&str]) -> process::Output {
        if log_enabled!(Level::Debug) {
            let cmd = format!(
                "tmux -L {} {}",
                self.socket_name,
                args.join(" ").to_string()
            );
            debug!("Tmux command: {}", cmd);
        }
        process::Command::new("tmux")
            .args(["-L", &self.socket_name])
            .args(args)
            .stdin(process::Stdio::inherit())
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute tmux command `{args:?}`"))
    }
}

impl Mux for Tmux {
    fn list_sessions(&self) -> Vec<String> {
        self.execute_tmux_command(&["list-sessions", "-F", "#S"])
            .output_to_string()
            .trim()
            .split('\n')
            .map(|x| x.to_string())
            .collect_vec()
    }

    fn session_exists(&self, name: &str) -> bool {
        self.execute_tmux_command(&["has-session", "-t", name])
            .status
            .success()
    }

    fn create_session(
        &self,
        name: &str,
        path: &Path,
        window_name: Option<&str>,
    ) -> miette::Result<()> {
        let mut args = vec![
            "new-session",
            "-d",
            "-s",
            name,
            "-c",
            path.to_str().expect("Path should be a utf-8 string"),
        ];

        if let Some(window_name) = window_name {
            args.extend(["-n", window_name]);
        }

        self.execute_tmux_command(&args).to_result()
    }

    fn attach_session(&self, name: &str) -> miette::Result<()> {
        if is_in_tmux() {
            self.execute_tmux_command(&["switch-client", "-t", name])
                .to_result()
        } else {
            self.execute_tmux_command(&["attach-session", "-t", name])
                .to_result()
        }
    }

    fn kill_session(&self, name: &str) -> miette::Result<()> {
        self.execute_tmux_command(&["kill-session", "-t", name])
            .to_result()
    }

    fn create_window(&self, name: &str, path: Option<&Path>) -> miette::Result<()> {
        let mut args = vec!["new-window", "-n", name];

        if let Some(path) = path {
            args.extend(&["-c", path.to_str().expect("Path should be a utf-8 string")]);
        }

        self.execute_tmux_command(&args).to_result()
    }

    fn send_command(&self, name: &str, command: &str) -> miette::Result<()> {
        self.execute_tmux_command(&["send-keys", "-t", name, command, "Enter"])
            .to_result()
    }

    fn session_name(&self) -> Option<String> {
        let output = self.execute_tmux_command(&["display-message", "-p", "#S"]);

        if output.status.success() {
            Some(output.output_to_string())
        } else {
            None
        }
    }

    fn create_or_attach(&self, name: &str, path: &Path) -> miette::Result<()> {
        if self.session_exists(name) {
            self.attach_session(name)
        } else {
            self.create_session(name, path, None)?;
            self.attach_session(name)
        }
    }
}

fn is_in_tmux() -> bool {
    std::env::var("TMUX").is_ok()
}
