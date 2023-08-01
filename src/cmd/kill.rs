use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    cmd::cli::Kill,
    config::Config,
    finder::{self, FinderOptions},
    tmux,
};

use super::Run;

impl Run for Kill {
    fn run(self) -> miette::Result<()> {
        let names = tmux::session_names()?;
        let selected = self.get_name(names)?;

        for sel in selected {
            tmux::kill_session(sel.as_str())?;
            println!("Killed {}", &sel);
        }

        Ok(())
    }
}

impl Kill {
    fn get_name(&self, names: Vec<String>) -> miette::Result<Vec<String>> {
        let config = Config::load()?;
        let query = self.query.as_ref().map(|v| v.join(" "));

        if self.all {
            return Ok(names);
        }

        // TODO: handle query better or remove it
        if let Some(query) = query.as_ref() {
            let matches = names
                .par_iter()
                .filter(|v| v.contains(query))
                .collect::<Vec<_>>();

            if matches.len() == 1 {
                return Ok(vec![matches
                    .first()
                    .expect("matches length is 1")
                    .to_string()]);
            }
        }

        let opts = FinderOptions {
            query: query.as_deref(),
            mode: config.mode,
            ..Default::default()
        };

        let result = finder::select_multi(&names, opts)?;
        Ok(result.unwrap_or_default())
    }
}
