use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{cmd::cli::Attach, config::Config, finder::FinderOptions, tmux, util};

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
                query,
                height: Some(config.height),
                ..Default::default()
            };

            let selected = match names.len() {
                0 => None,
                1 => names.into_iter().next(),
                _ => config
                    .finder
                    .execute(names.iter(), opts)?
                    .into_iter()
                    .next(),
            };

            if let Some(selected) = selected {
                tmux::attach_session(selected.as_str())?;
            }

            return Ok(());
        }

        let paths = config.list_paths();
        let selected = match get_selected(&paths, &query, &self, &config) {
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

fn get_selected(
    paths: &Vec<String>,
    query: &Option<String>,
    attach: &Attach,
    config: &Config,
) -> Result<Option<PathBuf>> {
    if let Some(path) = attach.path.as_ref() {
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
        exact: attach.exact,
        query: query.clone(),
        height: Some(config.height),
        ..Default::default()
    };

    Ok(config
        .finder
        .execute(paths.iter(), opts)?
        .into_iter()
        .next()
        .map(PathBuf::from))
}
