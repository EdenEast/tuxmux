use std::path::Path;

use miette::Result;

mod tmux;
mod zellij;
pub use tmux::Tmux;
pub use zellij::Zellij;

#[derive(Debug, Default, Clone, Copy)]
pub enum Mux {
    #[default]
    Zellij,
    Tmux,
}

pub trait Multiplexer {
    fn list_sessions(&self) -> Vec<String>;
    fn session_exists(&self, name: &str) -> bool;
    fn create_session(&self, name: &str, path: &Path) -> Result<()>;
    fn attach_session(&self, name: &str) -> Result<()>;
    fn kill_session(&self, name: &str) -> Result<()>;
    fn send_command(&self, name: &str, command: &str) -> Result<()>;

    fn create_or_attach(&self, name: &str, path: &Path) -> Result<()> {
        if self.session_exists(name) {
            self.attach_session(name)
        } else {
            self.create_session(name, path)?;
            self.attach_session(name)
        }
    }
}

impl Multiplexer for Mux {
    fn list_sessions(&self) -> Vec<String> {
        match self {
            Mux::Zellij => Zellij::default().list_sessions(),
            Mux::Tmux => Tmux::default().list_sessions(),
        }
    }

    fn session_exists(&self, name: &str) -> bool {
        match self {
            Mux::Zellij => Zellij::default().session_exists(name),
            Mux::Tmux => Tmux::default().session_exists(name),
        }
    }

    fn create_session(&self, name: &str, path: &Path) -> Result<()> {
        match self {
            Mux::Zellij => Zellij::default().create_session(name, path),
            Mux::Tmux => Tmux::default().create_session(name, path),
        }
    }

    fn attach_session(&self, name: &str) -> Result<()> {
        match self {
            Mux::Zellij => Zellij::default().attach_session(name),
            Mux::Tmux => Tmux::default().attach_session(name),
        }
    }

    fn kill_session(&self, name: &str) -> Result<()> {
        match self {
            Mux::Zellij => Zellij::default().kill_session(name),
            Mux::Tmux => Tmux::default().kill_session(name),
        }
    }

    fn send_command(&self, name: &str, command: &str) -> Result<()> {
        match self {
            Mux::Zellij => Zellij::default().send_command(name, command),
            Mux::Tmux => Tmux::default().send_command(name, command),
        }
    }
}
