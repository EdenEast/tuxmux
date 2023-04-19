use crate::{cmd::cli::Kill, config::Config, finder::FinderOptions, tmux};

use super::Run;

impl Run for Kill {
    fn run(self) -> eyre::Result<()> {
        let config = Config::load()?;
        let query = self.query.as_ref().map(|v| v.join(" "));

        let names = tmux::session_names()?;
        let selected = if self.all {
            names
        } else {
            let opts = FinderOptions {
                query,
                multi: true,
                height: Some(config.height),
                ..Default::default()
            };
            config.finder.execute(names.iter(), opts)?
        };

        for sel in selected {
            tmux::kill_session(&sel)?;
            println!("Killed {}", sel);
        }

        Ok(())
    }
}
