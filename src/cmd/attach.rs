use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{cli::Attach, data::Settings, finder::FinderOptions, tmux, util};

use eyre::Result;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::ExecuteableCmd;

impl ExecuteableCmd for Attach {
    fn execute(self) -> eyre::Result<()> {
        let settings = Settings::new()?;
        let query = self.query.as_ref().map(|v| v.join(" "));

        if self.exists {
            let names = crate::tmux::session_names()?;
            let opts = FinderOptions {
                exact: self.exact,
                query,
                height: settings.height,
                ..Default::default()
            };

            let selected = match names.len() {
                0 => None,
                1 => names.into_iter().next(),
                _ => settings
                    .finder()
                    .execute(names.iter(), opts)?
                    .into_iter()
                    .next(),
            };

            if let Some(selected) = selected {
                tmux::attach_session(selected.as_str())?;
            }

            return Ok(());
        }

        let paths = settings.list_paths();
        let selected = match get_selected(&paths, &query, &self, &settings) {
            Ok(Some(s)) => s,
            Ok(None) => return Ok(()),
            Err(e) => return Err(e),
        };

        execute_selected(&selected)
    }
}

pub fn use_cwd() -> Result<()> {
    execute_selected(&std::env::current_dir()?)
}

fn execute_selected(selected: &Path) -> Result<()> {
    let name = util::format_name(selected.file_name().unwrap().to_str().unwrap());
    tmux::create_or_attach_session(&name, selected.to_str().unwrap())
}

fn get_selected(
    paths: &HashSet<String>,
    query: &Option<String>,
    attach: &Attach,
    settings: &Settings,
) -> Result<Option<PathBuf>> {
    if let Some(path) = attach.path.as_ref() {
        if path.as_path() == Path::new(".") {
            return Ok(Some(std::env::current_dir()?));
        }

        if !path.exists() {
            return Err(eyre::eyre!("Invalid path: '{}'", path.display()));
        }

        return Ok(Some(path.to_owned()));
    }

    if let Some(query) = &query {
        let iter = paths.par_iter().filter(|v| v.contains(query));
        let count = iter.clone().count();
        if count == 1 {
            let r = iter.collect::<Vec<_>>()[0];
            return Ok(Some(PathBuf::from_str(&r)?));
        }
    }

    let opts = FinderOptions {
        exact: attach.exact,
        query: query.clone(),
        height: settings.height,
        ..Default::default()
    };

    Ok(settings
        .finder()
        .execute(paths.iter(), opts)?
        .into_iter()
        .next()
        .map(PathBuf::from))
}
