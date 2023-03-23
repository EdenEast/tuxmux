use crate::{cli::Kill, data::Settings, finder::FinderOptions, tmux};

use super::ExecuteableCmd;

impl ExecuteableCmd for Kill {
    fn execute(self) -> eyre::Result<()> {
        let settings = Settings::new()?;
        let query = self.query.as_ref().map(|v| v.join(" "));

        let names = tmux::session_names()?;
        let selected = if self.all {
            names
        } else {
            let opts = FinderOptions {
                query,
                multi: true,
                height: settings.height,
                ..Default::default()
            };
            settings.finder().execute(names.iter(), opts)?
        };

        for sel in selected {
            tmux::kill_session(&sel)?;
            println!("Killed {}", sel);
        }

        Ok(())
    }
}
