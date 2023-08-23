use crate::{cmd::cli::List, config::Config, tmux, walker::Walker};

use super::Run;

impl Run for List {
    fn run(self) -> miette::Result<()> {
        if self.all {
            let config = Config::load()?;

            for path in config.paths_from_walk() {
                println!("{}", path);
            }

            return Ok(());
        }

        let sessions = tmux::sessions()?;

        let max_name = sessions
            .iter()
            .map(|s| s.name.as_ref().map(|v| v.len()).unwrap_or(0))
            .max()
            .unwrap_or_default();

        for s in sessions {
            let attach_num = s.attached.unwrap_or(0);
            let name = s.name.unwrap_or_default();

            let attach = if attach_num > 0 {
                attach_num.to_string()
            } else {
                " ".to_string()
            };

            println!("{} {:npad$}", attach, name, npad = max_name);
        }

        Ok(())
    }
}
