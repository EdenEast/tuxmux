use eyre::{Context, Result};
use tmux_interface::{
    AttachSession, HasSession, KillSession, NewSession, NewWindow, SendKeys, Session, Sessions,
    SwitchClient, TmuxCommand, SESSION_ALL,
};

pub fn sessions() -> Result<Vec<Session>> {
    Ok(Sessions::get(SESSION_ALL)?.into_iter().collect())
}

pub fn session_names() -> Result<Vec<String>> {
    let tmux_sessions = Sessions::get(SESSION_ALL)?;

    Ok(tmux_sessions
        .into_iter()
        .map(|t| {
            t.name
                .unwrap_or_else(|| t.id.map(|i| i.to_string()).unwrap_or_else(|| "".to_owned()))
        })
        .collect::<Vec<String>>())
}

pub fn session_exists(name: &str) -> bool {
    match HasSession::new().target_session(name).output() {
        Ok(output) => output.success(),
        _ => false,
    }
}

pub fn create_or_attach_session(name: &str, path: &str) -> Result<()> {
    if session_exists(name) {
        attach_session(name)
    } else {
        create_session(name, path)?;
        attach_session(name)
    }
}

pub fn attach_session(name: &str) -> Result<()> {
    if in_tmux() {
        SwitchClient::new()
            .target_session(name)
            .output()
            .wrap_err_with(|| format!("Failed to switch to session {}", name))?;
    } else {
        AttachSession::new()
            .target_session(name)
            .output()
            .wrap_err_with(|| format!("Failed to attach to session {}", name))?;
    }

    Ok(())
}

pub fn create_session(name: &str, path: &str) -> Result<()> {
    NewSession::new()
        .detached()
        .session_name(name)
        .start_directory(path)
        .output()
        .wrap_err_with(|| format!("Failed to create session `{}`", name))?;

    Ok(())
}

pub fn create_window(name: &str) -> Result<()> {
    NewWindow::new()
        .detached()
        .window_name(name)
        .output()
        .wrap_err_with(|| format!("Failed to create window `{}`", name))?;

    Ok(())
}

pub fn send_keys(target: &str, key: &str) -> Result<()> {
    SendKeys::new()
        .target_pane(target)
        .key(key)
        .output()
        .wrap_err_with(|| format!("Failed to send '{}' to pane `{}`", key, target))?;

    Ok(())
}

pub fn session_name() -> String {
    match TmuxCommand::new()
        .cmd("display-message")
        .push_flag("-p")
        .push_param("#S")
        .output()
    {
        Ok(out) => out.to_string(),
        _ => String::new(),
    }
}

pub fn kill_session(name: &str) -> Result<()> {
    KillSession::new()
        .target_session(name)
        .output()
        .wrap_err_with(|| format!("Failed to kill session `{}`", name))?;

    Ok(())
}

fn in_tmux() -> bool {
    std::env::var("TMUX").is_ok()
}
