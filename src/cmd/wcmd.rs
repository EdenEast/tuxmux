use std::path::Path;

use crate::{cmd::cli::Wcmd, tmux, util::intersperse};

use super::Run;

impl Run for Wcmd {
    fn run(self) -> eyre::Result<()> {
        let name = Path::new(&self.window)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let session_name = tmux::session_name();
        let target = format!("{}:{}", session_name.trim(), name);

        if !tmux::session_exists(&target) {
            tmux::create_window(name)?;
        }

        let cmd: String = intersperse(self.cmds.iter().map(|f| f.as_str()), " ").collect();

        tmux::send_keys(&target, &cmd)?;
        tmux::send_keys(&target, "C-m")?;

        Ok(())
    }
}
