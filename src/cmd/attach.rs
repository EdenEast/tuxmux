use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    cmd::cli::Attach,
    config::Config,
    finder::{self, FinderOptions},
    mux::Multiplexer,
    util,
    walker::Walker,
};

use dialoguer::{
    console::{Style, Term},
    theme::ColorfulTheme,
    FuzzySelect,
};
use gix::bstr::ByteSlice;
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
                // exact: self.exact,
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
                config.mux.attach_session(selected.as_str())?;
            }

            return Ok(());
        }

        if let Some(path) = self.path.as_ref() {
            if path.as_path() == Path::new(".") {
                let cwd = std::env::current_dir().into_diagnostic()?;
                return self.execute_selected(&cwd, &config);
            }

            if !path.exists() {
                return Err(miette!("Path does not exist: '{}'", path.display()));
            }

            return self.execute_selected(path, &config);
        }

        let paths = config.paths_from_walk();

        if let Some(query) = query.as_ref() {
            // Check if there is one exact match if so then execute that
            let matches = paths
                .par_iter()
                .filter(|v| v.contains(query))
                .collect::<Vec<_>>();
            if matches.len() == 1 {
                return self.execute_selected(
                    &PathBuf::from_str(matches.first().expect("Matches length is checked to be 1"))
                        .into_diagnostic()?,
                    &config,
                );
            }
        }

        let theme = ColorfulTheme {
            fuzzy_match_highlight_style: Style::new().for_stderr().red(),
            ..Default::default()
        };

        let term = Term::stderr();
        let (rows, _) = term.size();
        let h = match config.mode {
            crate::config::Mode::Full => rows as usize,
            crate::config::Mode::Lines(lines) => lines as usize,
            crate::config::Mode::Percentage(percentage) => (rows as f32 * percentage) as usize,
        };

        let selection = FuzzySelect::with_theme(&theme)
            .default(0)
            .items(&paths)
            .with_initial_text(query.unwrap_or_default())
            .max_length(h)
            .interact_on_opt(&term)
            .into_diagnostic()?;

        match selection {
            Some(index) => {
                let selected = PathBuf::from(&paths[index]);
                self.execute_selected(&selected, &config)
            }
            None => Ok(()),
        }
    }
}

impl Attach {
    fn execute_selected(&self, selected: &Path, config: &Config) -> Result<()> {
        let name = util::format_name(selected.file_name().unwrap().to_str().unwrap());
        if config.mux.session_exists(&name) {
            return config.mux.attach_session(&name);
        }

        let get_worktree = || -> Option<PathBuf> {
            let repo = gix::open(selected).ok()?;

            // TODO: add configuration option for toggling this option
            if !repo.config_snapshot().boolean("core.bare").unwrap_or(false) {
                return None;
            }

            let trees = repo.worktrees().ok()?;
            let items = trees.iter().map(|t| t.id()).collect_vec();
            let use_default = self.default || config.default_worktree;
            let selected_index = match items.len() {
                0 => None,
                1 => Some(0),
                _ => {
                    let index = if use_default {
                        default_branch(&repo)
                            .and_then(|name| {
                                let s = name.as_str();
                                items.iter().find_position(|item| **item == s)
                            })
                            .map(|(i, _)| i)
                    } else {
                        None
                    };

                    match index {
                        Some(index) => Some(index),
                        None => {
                            let theme = ColorfulTheme {
                                fuzzy_match_highlight_style: Style::new().for_stderr().red(),
                                ..Default::default()
                            };
                            let selected = FuzzySelect::with_theme(&theme)
                                .with_prompt("Worktree")
                                .default(0)
                                .items(&items)
                                .interact_opt()
                                .ok()?;

                            // The user cancelled the selection, return a different exit code
                            if selected.is_none() {
                                std::process::exit(2);
                            }

                            selected
                        }
                    }
                }
            };

            selected_index.and_then(|i| trees.get(i).expect("string comes from repo").base().ok())
        };

        config.mux.create_session(&name, selected)?;
        if let Some(worktree) = get_worktree() {
            config
                .mux
                .send_command(&name, &format!("cd {}", worktree.path().display()))?;
        }
        config.mux.attach_session(&name)?;

        Ok(())
    }

    pub fn use_cwd(&self, config: &Config) -> Result<()> {
        self.execute_selected(&std::env::current_dir().into_diagnostic()?, config)
    }
}

fn default_branch(repo: &gix::Repository) -> Option<String> {
    let remote = repo
        .find_default_remote(gix::remote::Direction::Fetch)?
        .ok()?;
    let name = remote.name()?.as_bstr().to_str().ok()?;
    let reference = repo.find_reference(&format!("{name}/HEAD")).ok()?;

    Some(
        reference
            .follow()?
            .ok()?
            .name()
            .shorten()
            .get(name.len() + 1..)?
            .to_str_lossy()
            .to_string(),
    )
}
