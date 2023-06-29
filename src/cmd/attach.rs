use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    cmd::cli::Attach,
    config::Config,
    finder::{self, FinderOptions},
    tmux, util,
    walker::Walker,
};

use miette::{miette, IntoDiagnostic, Result};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::Run;

impl Run for Attach {
    fn run(self) -> Result<()> {
        let config = Config::load()?;
        let query = self.query.as_ref().map(|v| v.join(" "));

        if self.exists {
            let names = crate::tmux::session_names()?;
            let opts = FinderOptions {
                exact: self.exact,
                query: query.as_deref(),
                mode: config.mode,
                ..Default::default()
            };

            let selected = match names.len() {
                0 => None,
                1 => names.into_iter().next(),
                _ => finder::find(names.iter(), opts).into_iter().next(),
            };

            if let Some(selected) = selected {
                tmux::attach_session(selected.as_str())?;
            }

            return Ok(());
        }

        let paths = config.paths_from_walk();
        let selected = match self.get_selected(&paths, &query, &config) {
            Ok(Some(s)) => s,
            Ok(None) => return Ok(()),
            Err(e) => return Err(e),
        };

        execute_selected(&selected)
    }
}

pub fn use_cwd() -> Result<()> {
    execute_selected(&std::env::current_dir().into_diagnostic()?)
}

fn execute_selected(selected: &Path) -> Result<()> {
    let name = util::format_name(selected.file_name().unwrap().to_str().unwrap());
    tmux::create_or_attach_session(&name, selected.to_str().unwrap())
}

impl Attach {
    fn get_selected(
        &self,
        paths: &Vec<String>,
        query: &Option<String>,
        config: &Config,
    ) -> Result<Option<PathBuf>> {
        if let Some(path) = self.path.as_ref() {
            if path.as_path() == Path::new(".") {
                return Ok(Some(std::env::current_dir().into_diagnostic()?));
            }

            if !path.exists() {
                return Err(miette!("Invalid path: '{}'", path.display()));
            }

            return Ok(Some(path.to_owned()));
        }

        if let Some(query) = &query {
            let matches = paths
                .par_iter()
                .filter(|v| v.contains(query))
                .collect::<Vec<_>>();
            if matches.len() == 1 {
                return Ok(Some(
                    PathBuf::from_str(matches.first().expect("Matches length is 1"))
                        .into_diagnostic()?,
                ));
            }
        }

        let opts = FinderOptions {
            exact: self.exact,
            query: query.as_deref(),
            mode: config.mode.clone(),
            ..Default::default()
        };

        Ok(finder::find(paths.iter(), opts)
            .into_iter()
            .next()
            .map(PathBuf::from))

        // Ok(finder::find(paths.iter(), opts)
        //     .map(|r| r.into_iter().next().map(PathBuf::from).unwrap_or_default()))
    }
}
