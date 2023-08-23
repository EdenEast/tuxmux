use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

use git2::Repository;

use crate::config::Config;

pub trait Walker {
    fn paths_from_walk(&self) -> Vec<String>;
}

impl Walker for Config {
    fn paths_from_walk(&self) -> Vec<String> {
        let mut result = self.search.single.clone();

        let exclude_paths = Arc::new(self.exclude_path.clone());

        let is_repository = |path: &Path| -> Option<Repository> {
            let file_name = path.file_name();
            if file_name == Some(OsStr::new(".git")) || file_name == Some(OsStr::new("git")) {
                Repository::open(path).ok()
            } else {
                None
            }
        };

        for workspace in &self.search.workspace {
            let exclude = exclude_paths.clone();

            let walk = jwalk::WalkDirGeneric::<((), Option<PathBuf>)>::new(Path::new(workspace))
                .follow_links(false)
                .skip_hidden(false);

            let additions = walk
                .process_read_dir(move |_depth, path, _read_dir_state, siblings| {
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
                        let epath = entry.path();
                        if let Some(repo) = is_repository(&epath) {
                            entry.client_state = Some(path.to_path_buf());
                            entry.read_children_path = None;

                            // If a bare repository but also has worktrees when we are most likly
                            // using a bare worktree structured repo:
                            //
                            // - .bare  -> Main git_dir folder
                            // - main -> worktree folder
                            // - feat1 -> worktree folder
                            // - feat2 -> worktree folder
                            // - .git -> file that points to `.bare`
                            let is_bare = {
                                let empty_trees =
                                    repo.worktrees().map(|t| t.is_empty()).unwrap_or(false);
                                repo.is_bare() && empty_trees
                            };

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
                        .map(|state| state.display().to_string())
                });

            result.extend(additions);
        }

        result
    }
}
