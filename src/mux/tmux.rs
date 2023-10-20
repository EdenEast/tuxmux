use tmux_interface::ListSessions;

use super::Multiplexer;

#[derive(Debug, Default, Clone, Copy)]
pub struct Tmux;

impl Multiplexer for Tmux {
    fn list_sessions(&self) -> Vec<String> {
        let output = match ListSessions::new().output() {
            Ok(output) => output,
            Err(_) => return Vec::new(),
        };

        dbg!(output);

        Vec::new()
    }

    fn session_exists(&self, name: &str) -> bool {
        todo!()
    }

    fn create_session(&self, name: &str, path: &std::path::Path) -> miette::Result<()> {
        todo!()
    }

    fn attach_session(&self, name: &str) -> miette::Result<()> {
        todo!()
    }

    fn kill_session(&self, name: &str) -> miette::Result<()> {
        todo!()
    }

    fn send_command(&self, name: &str, command: &str) -> miette::Result<()> {
        todo!()
    }
}
