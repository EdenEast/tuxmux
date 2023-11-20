use std::{borrow::Cow, path::Path};

use itertools::Itertools;
use miette::{IntoDiagnostic, Result};
use tmux_interface::{
    AttachSession, DisplayMessage, HasSession, KillSession, ListSessions, NewSession, NewWindow,
    SendKeys, SwitchClient, Tmux,
};

pub fn list_sessions() -> Vec<String> {
    let output = match Tmux::with_command(ListSessions::new().format("#S")).output() {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };

    String::from_utf8(output.stdout())
        .map(|s| s.lines().map(|s| s.to_string()).collect_vec())
        .unwrap_or_default()
}

pub fn session_exists(name: &str) -> bool {
    Tmux::with_command(HasSession::new().target_session(name))
        .output()
        .map(|out| out.success())
        .unwrap_or(false)
}

pub fn create_session(name: &str, path: &Path, window_name: Option<&str>) -> Result<()> {
    let mut command = NewSession::new()
        .detached()
        .session_name(name)
        .start_directory(path.to_string_lossy());
    command.window_name = window_name.map(Cow::Borrowed);
    Tmux::with_command(command).output().into_diagnostic()?;
    Ok(())
}

pub fn attach_session(name: &str) -> Result<()> {
    if in_tmux() {
        Tmux::with_command(SwitchClient::new().target_session(name))
            .output()
            .into_diagnostic()?;
    } else {
        Tmux::with_command(AttachSession::new().target_session(name))
            .output()
            .into_diagnostic()?;
    }
    Ok(())
}

pub fn kill_session(name: &str) -> Result<()> {
    Tmux::with_command(KillSession::new().target_session(name))
        .output()
        .into_diagnostic()?;
    Ok(())
}

pub fn create_window(name: &str, path: Option<&Path>) -> Result<()> {
    let window = match path {
        Some(path) => NewWindow::new()
            .window_name(name)
            .start_directory(path.to_string_lossy()),
        None => NewWindow::new().window_name(name),
    };
    Tmux::with_command(window).output().into_diagnostic()?;
    Ok(())
}

pub fn send_command(name: &str, command: &str) -> Result<()> {
    Tmux::with_command(SendKeys::new().target_pane(name).key(command))
        .output()
        .into_diagnostic()?;
    Tmux::with_command(SendKeys::new().target_pane(name).key("C-m"))
        .output()
        .into_diagnostic()?;
    Ok(())
}

pub fn session_name() -> Option<String> {
    Tmux::with_command(DisplayMessage::new().print().message("#S"))
        .output()
        .into_diagnostic()
        .and_then(|out| String::from_utf8(out.stdout()).into_diagnostic())
        .ok()
}

fn in_tmux() -> bool {
    std::env::var("TMUX").is_ok()
}
