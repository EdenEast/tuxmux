use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

use gix::repository::Kind;

use crate::config::Config;

pub trait Walker {
    fn paths_from_walk(&self) -> Vec<String>;
}

impl Walker for Config {
    fn paths_from_walk(&self) -> Vec<String> {
        let mut result = self.search.single.clone();
        let exclude_paths = Arc::new(self.exclude_path.clone());

        for workspace in &self.search.workspace {
            let exclude = exclude_paths.clone();

            let walk = jwalk::WalkDirGeneric::<((), Option<Kind>)>::new(Path::new(workspace))
                .follow_links(false)
                .skip_hidden(false);

            let additions = walk
                .process_read_dir(move |_depth, _path, _read_dir_state, siblings| {
                    siblings.retain(|entry_result| {
                        entry_result
                            .as_ref()
                            .map(|entry| {
                                entry
                                    .path()
                                    .components()
                                    .last()
                                    .expect("always has last component")
                                    .as_os_str()
                                    .to_str()
                                    .map(|name| !exclude.iter().any(|e| *e == name))
                                    .unwrap_or(false)
                            })
                            .unwrap_or(false)
                    });

                    let mut found_any_repo = false;
                    let mut found_bare_repo = false;
                    for entry in siblings.iter_mut().flatten() {
                        let path = entry.path();
                        if let Some(kind) = is_repository(&path) {
                            let is_bare = kind.is_bare();
                            entry.client_state = kind.into();
                            entry.read_children_path = None;

                            found_any_repo = true;
                            found_bare_repo = is_bare;
                        }
                    }
                    // Only return paths which are repositories are further participating in the traversal
                    // Don't let bare repositories cause siblings to be pruned.
                    if found_any_repo && !found_bare_repo {
                        siblings.retain(|e| {
                            e.as_ref()
                                .map(|e| e.client_state.is_some())
                                .unwrap_or(false)
                        });
                    }
                })
                .into_iter()
                .filter_map(Result::ok)
                .filter_map(|mut e| {
                    e.client_state
                        .take()
                        .map(|state| into_workdir(e.path(), &state).display().to_string())
                });

            result.extend(additions);
        }

        result
    }
}

fn is_repository(path: &Path) -> Option<Kind> {
    // Can be git dir or worktree checkout (file)
    if path.file_name() != Some(OsStr::new(".git")) && path.extension() != Some(OsStr::new("git")) {
        return None;
    }

    if path.is_dir() {
        if path.join("HEAD").is_file() && path.join("config").is_file() {
            gix::discover::is_git(path).ok().map(Into::into)
        } else {
            None
        }
    } else {
        // git files are always worktrees
        Some(Kind::WorkTree { is_linked: true })
    }
}

fn into_workdir(git_dir: PathBuf, kind: &Kind) -> PathBuf {
    if matches!(kind, Kind::Bare) || gix::discover::is_bare(&git_dir) {
        git_dir
    } else {
        git_dir
            .parent()
            .expect("git is never in the root")
            .to_owned()
    }
}
