use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    cmd::cli::Attach,
    config::{Config, WorktreeMode},
    finder::{self, FinderOptions},
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
        let mux = &config.mux;
        let query = self.query.as_ref().map(|v| v.join(" "));

        if self.exists {
            let names = mux.list_sessions();
            let opts = FinderOptions {
                // exact: self.exact,
                query: query.as_deref(),
                mode: config.mode,
                ..Default::default()
            };

            let selected = match names.len() {
                0 => None,
                1 => names.into_iter().next(),
                _ => finder::find(&names, opts)?,
            };

            if let Some(selected) = selected {
                mux.attach_session(&selected)?;
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
        let mux = &config.mux;
        let name = util::format_name(selected.file_name().unwrap().to_str().unwrap());
        if mux.session_exists(&name) {
            return mux.attach_session(&name);
        }

        let repo = gix::open(selected).ok();
        // Check worktree mode
        if let Some(repo) = repo {
            let worktrees = repo.worktrees().into_diagnostic()?;
            let is_bare = is_bare(&repo);

            if config.worktree_mode == WorktreeMode::All || self.all {
                let mut iter = worktrees.iter();

                // A bare repo that contains worktrees only uses worktrees and does not contain
                // a normally contain a valid work_dir. When creating the session use the first
                // worktree as the window name.
                if is_bare && !worktrees.is_empty() {
                    let first = iter.next();
                    let path = first.and_then(|tree| tree.base().ok());
                    let window_name = first.map(|f| f.id().to_string());
                    mux.create_session(
                        &name,
                        path.expect("worktrees is not empty"),
                        window_name.as_deref(),
                    )?;
                } else {
                    let head_branch = head_branch(&repo);
                    mux.create_session(
                        &name,
                        selected.to_string_lossy().as_ref(),
                        head_branch.as_deref(),
                    )?;
                };

                for tree in iter {
                    let path = tree.base().ok();
                    let window_name = tree.id().to_str_lossy();
                    mux.create_window(&window_name, path.as_deref())?;
                }

                return mux.attach_session(&name);
            } else if config.worktree_mode == WorktreeMode::Default || self.default {
                let len = worktrees.len();
                // If the repo is not bare then the default would be the work_dir of the repo.
                // This will be used instead of any worktree
                if !is_bare || len == 0 {
                    mux.create_session(&name, selected.to_string_lossy().as_ref(), None)?;
                    return mux.attach_session(&name);
                }

                // If there is only one worktree and it is a bare repo it is the only option
                if len == 1 {
                    let proxy = worktrees.first().expect("worktree contains values");
                    let path = proxy.base().into_diagnostic()?;
                    let proxy_repo = proxy
                        .clone()
                        .into_repo_with_possibly_inaccessible_worktree()
                        .into_diagnostic()?;
                    let window_name = head_branch(&proxy_repo);
                    mux.create_session(&name, path, window_name.as_deref())?;
                    return mux.attach_session(&name);
                }

                if let Some(default_branch) = default_branch(&repo) {
                    if let Some(default_worktree) =
                        worktrees.iter().find(|tree| tree.id() == default_branch)
                    {
                        let path = default_worktree.base().into_diagnostic()?;
                        mux.create_session(&name, path, Some(&default_branch))?;
                        return mux.attach_session(&name);
                    }
                }

                return Err(miette!("Could not find default branch / worktree"));
            } else {
                let items = worktrees
                    .iter()
                    .map(|tree| tree.id().to_string())
                    .collect_vec();

                if let Some(choice) = fuzzy(&items, "Worktree") {
                    let tree = worktrees
                        .get(choice)
                        .expect("choice value comes from worktree index");
                    let path = tree.base().into_diagnostic()?;
                    let window_name = tree.id();
                    mux.create_session(&name, path, Some(window_name.to_str_lossy().as_ref()))?;
                    return mux.attach_session(&name);
                }
            }
        } else {
            // This is not a git repo so just create and attach.
            mux.create_session(&name, selected.to_str().unwrap(), None)?;
            return mux.attach_session(&name);
        }

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

fn head_branch(repo: &gix::Repository) -> Option<String> {
    repo.head()
        .ok()?
        .referent_name()
        .map(|r| r.shorten().to_string())
}

fn is_bare(repo: &gix::Repository) -> bool {
    repo.config_snapshot()
        .boolean("core.bare")
        .unwrap_or_default()
}

fn fuzzy<T: ToString>(items: &[T], prompt: &str) -> Option<usize> {
    let theme = ColorfulTheme {
        fuzzy_match_highlight_style: Style::new().for_stderr().red(),
        ..Default::default()
    };
    let selected = FuzzySelect::with_theme(&theme)
        .with_prompt(prompt)
        .default(0)
        .items(items)
        .interact_opt()
        .ok()?;

    // The user cancelled the selection, return a different exit code
    if selected.is_none() {
        std::process::exit(2);
    }

    selected
}
