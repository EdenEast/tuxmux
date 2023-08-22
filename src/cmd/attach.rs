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

use dialoguer::{
    console::{Style, Term},
    theme::ColorfulTheme,
    FuzzySelect, Select,
};
use git2::Repository;
use itertools::Itertools;
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
                _ => finder::find(&names, opts)?, // _ => finder::find(names.iter(), opts).into_iter().next(),
            };

            if let Some(selected) = selected {
                tmux::attach_session(selected.as_str())?;
            }

            return Ok(());
        }

        if let Some(path) = self.path.as_ref() {
            if path.as_path() == Path::new(".") {
                let cwd = std::env::current_dir().into_diagnostic()?;
                return execute_selected(&cwd);
            }

            if !path.exists() {
                return Err(miette!("Path does not exist: '{}'", path.display()));
            }

            return execute_selected(path);
        }

        let paths = config.paths_from_walk();

        if let Some(query) = query.as_ref() {
            // Check if there is one exact match if so then execute that
            let matches = paths
                .par_iter()
                .filter(|v| v.contains(query))
                .collect::<Vec<_>>();
            if matches.len() == 1 {
                return execute_selected(
                    &PathBuf::from_str(matches.first().expect("Matches length is checked to be 1"))
                        .into_diagnostic()?,
                );
            }
        }

        let mut theme = ColorfulTheme::default();
        theme.fuzzy_match_highlight_style = Style::new().for_stderr().red();

        let term = Term::stderr();
        let (rows, _) = term.size();
        let h = match config.mode {
            crate::config::Mode::Full => rows as usize,
            crate::config::Mode::Lines(lines) => lines as usize,
            crate::config::Mode::Percentage(percentage) => (rows as f32 * percentage) as usize,
        };

        let selection = if self.exact {
            Select::with_theme(&theme)
                .default(0)
                .items(&paths)
                .max_length(h)
                .interact_on_opt(&term)
                .into_diagnostic()?
        } else {
            FuzzySelect::with_theme(&theme)
                .default(0)
                .items(&paths)
                .with_initial_text(query.unwrap_or_default())
                .max_length(h)
                .interact_on_opt(&term)
                .into_diagnostic()?
        };

        match selection {
            Some(index) => {
                let selected = PathBuf::from(&paths[index]);
                execute_selected(&selected)
            }
            None => Ok(()),
        }
    }
}

pub fn use_cwd() -> Result<()> {
    execute_selected(&std::env::current_dir().into_diagnostic()?)
}

fn execute_selected(selected: &Path) -> Result<()> {
    let name = util::format_name(selected.file_name().unwrap().to_str().unwrap());
    if tmux::session_exists(&name) {
        return tmux::attach_session(&name);
    }

    let worktree = if let Ok(repo) = Repository::open(selected) {
        if let Ok(trees) = repo.worktrees() {
            let items = trees.iter().filter_map(|e| e).collect_vec();
            let selected_index = match items.len() {
                0 => None,
                1 => Some(0),
                _ => {
                    let mut theme = ColorfulTheme::default();
                    theme.fuzzy_match_highlight_style = Style::new().for_stderr().red();
                    FuzzySelect::with_theme(&theme)
                        .default(0)
                        .items(&items)
                        .interact()
                        .into_diagnostic()
                        .ok()
                }
            };

            selected_index.map(|i| {
                repo.find_worktree(items[i])
                    .expect("string comes from repo")
            })
        } else {
            None
        }
    } else {
        None
    };

    tmux::create_session(&name, &selected.to_str().unwrap())?;
    if let Some(worktree) = worktree {
        tmux::send_command(&name, &format!("cd {}", worktree.path().display()))?;
    }
    tmux::attach_session(&name)?;

    Ok(())
}
