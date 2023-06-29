use crate::{
    cmd::cli::Kill,
    config::Config,
    finder::{self, FinderOptions},
    tmux,
};

use super::Run;

impl Run for Kill {
    fn run(self) -> miette::Result<()> {
        let config = Config::load()?;
        let query = self.query.map(|v| v.join(" "));

        let names = tmux::session_names()?;
        let selected = if self.all {
            names
        } else {
            let opts = FinderOptions {
                multi: true,
                query: query.as_deref(),
                mode: config.mode,
                ..Default::default()
            };

            finder::find(names.iter(), opts)
        };

        for sel in selected {
            tmux::kill_session(sel.as_str())?;
            println!("Killed {}", &sel);
        }

        Ok(())
    }
}
