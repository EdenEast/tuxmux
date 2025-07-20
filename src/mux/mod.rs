use std::path::Path;

use miette::{miette, Result};

pub mod tmux;

pub trait OutputExtension {
    fn output_to_string(self) -> String;
    fn to_result(self) -> Result<()>;
}

impl OutputExtension for std::process::Output {
    fn output_to_string(self) -> String {
        String::from_utf8(self.stdout)
            .expect("The output of a `tmux` command should always be valid utf-8")
    }

    fn to_result(self) -> Result<()> {
        if self.status.success() {
            return Ok(());
        }
        Err(miette!(
            "failed with {}",
            String::from_utf8(self.stderr).expect("tmux stderr is not valid utf-8")
        ))
    }
}

pub trait Mux: Sized {
    fn list_sessions(&self) -> Vec<String>;
    fn session_exists(&self, name: &str) -> bool;
    fn create_session(&self, name: &str, path: &Path, window_name: Option<&str>) -> Result<()>;
    fn attach_session(&self, name: &str) -> Result<()>;
    fn kill_session(&self, name: &str) -> Result<()>;
    fn create_window(&self, name: &str, path: Option<&Path>) -> Result<()>;
    fn send_command(&self, name: &str, command: &str) -> Result<()>;
    fn session_name(&self) -> Option<String>;
    fn create_or_attach(&self, name: &str, path: &Path) -> Result<()>;
}
