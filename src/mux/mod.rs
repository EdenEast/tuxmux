use std::path::Path;

use miette::Result;

mod tmux;

#[derive(Debug, Default)]
pub enum Mux {
    #[default]
    Tmux,
}

impl Mux {
    pub fn list_sessions(&self) -> Vec<String> {
        tmux::list_sessions()
    }

    pub fn session_exists(&self, name: &str) -> bool {
        tmux::session_exists(name)
    }

    pub fn create_session<P: AsRef<Path>>(
        &self,
        name: &str,
        path: P,
        window_name: Option<&str>,
    ) -> Result<()> {
        tmux::create_session(name, path.as_ref(), window_name)
    }

    pub fn attach_session(&self, name: &str) -> Result<()> {
        tmux::attach_session(name)
    }

    pub fn kill_session(&self, name: &str) -> Result<()> {
        tmux::kill_session(name)
    }

    pub fn create_window(&self, name: &str, path: Option<&Path>) -> Result<()> {
        tmux::create_window(name, path)
    }

    pub fn send_command(&self, name: &str, command: &str) -> Result<()> {
        tmux::send_command(name, command)
    }

    pub fn session_name(&self) -> Option<String> {
        tmux::session_name()
    }

    pub fn create_or_attach<P: AsRef<Path>>(&self, name: &str, path: P) -> Result<()> {
        if self.session_exists(name) {
            self.attach_session(name)
        } else {
            self.create_session(name, path.as_ref(), None)?;
            self.attach_session(name)
        }
    }
}
