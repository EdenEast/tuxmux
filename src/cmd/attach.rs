use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{cmd::cli::Attach, config::Config, ui::Picker, util, walker::Walker};

use gix::{bstr::ByteSlice, Repository};
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
            let selected = match names.len() {
                0 => None,
                1 => names.into_iter().next(),
                _ => Picker::new()
                    .items(&names)
                    .prompt("> ")
                    .filter(query.as_deref())
                    .select()?,
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

        // let mut picker = Picker::new(&paths, "> ".into());
        // let choice = match picker.get_selection()? {
        //     crate::ui::PickerSelection::Selection(s) => s,
        //     crate::ui::PickerSelection::ModifiedSelection(s) => s,
        //     crate::ui::PickerSelection::None => todo!(),
        // };

        let choice = match Picker::new()
            .items(&paths)
            .filter(query.as_deref())
            .prompt("> ")
            .select()?
        {
            Some(s) => s,
            None => return Ok(()),
        };

        let choice = Path::new(&choice);
        self.execute_selected(choice, &config)
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
        let worktree = self.get_worktree(repo.as_ref(), config);
        let branch = repo.as_ref().and_then(head_branch);
        mux.create_session(&name, selected.to_str().unwrap(), branch.as_deref())?;

        if let Some(worktree) = worktree {
            mux.send_command(&name, &format!("cd {}", worktree.display()))?;
        }

        mux.attach_session(&name)?;

        Ok(())
    }

    pub fn use_cwd(&self, config: &Config) -> Result<()> {
        self.execute_selected(&std::env::current_dir().into_diagnostic()?, config)
    }

    fn get_worktree(&self, repo: Option<&Repository>, config: &Config) -> Option<PathBuf> {
        let repo = repo?;
        let worktrees = repo.worktrees().ok()?;
        let use_default = self.default || config.default_worktree;
        let worktree_length = worktrees.len();
        let bare = is_bare(repo);

        if worktree_length == 0 {
            return None;
        }

        // If the repository is not bare then worktree's are in addition to the main default
        // worktree. If we are to use 'default' we should not use any worktrees
        if !bare && use_default {
            return None;
        }

        // NOTE: A worktree's id() (name) can be different then it's branch name. To get the branch
        // name you have to get the proxy repo and get the head branch of that.
        let items = worktrees.iter().map(|t| t.id().to_string()).collect_vec();
        if worktree_length == 1 {
            // If the repo is a bare repo then there is only one valid working tree
            if bare {
                return worktrees[0].base().ok();
            }

            let default_branch = head_branch(repo)?;
            let mut choices = vec![default_branch];
            choices.extend(items);

            let choice = Picker::new()
                .items(&choices)
                .prompt("Worktree: ")
                .select()
                .ok()??;

            let choice = Path::new(&choice);
            return worktrees
                .into_iter()
                .find(|proxy| proxy.git_dir() == choice)
                .and_then(|proxy| proxy.base().ok());
        }

        if use_default {
            return default_branch(repo)
                .and_then(|name| {
                    let s = name.as_str();
                    items.iter().position(|x| x == s)
                })
                .and_then(|index| worktrees[index].base().ok());
        }

        let choice = Picker::new()
            .items(&items)
            .prompt("Worktree: ")
            .select()
            .ok()??;

        let choice = Path::new(&choice);
        worktrees
            .into_iter()
            .find(|proxy| proxy.git_dir() == choice)
            .and_then(|proxy| proxy.base().ok())
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
