use eyre::Result;
use jwalk::WalkDir;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::{finder::FinderChoice, util};

const CONF_PATH_COMPONENTS: &[&str] = &["config.toml"];

lazy_static::lazy_static! {
    static ref HOME_DIR: PathBuf = dirs_next::home_dir().unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Global,
    Local,
}

#[derive(Debug, Clone, Copy)]
pub enum PathKind {
    Workspace,
    Single,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub single_paths: HashSet<String>,
    pub workspace_paths: HashSet<String>,
    pub depth: Option<usize>,
    pub height: Option<usize>,
    pub finder: Option<FinderChoice>,
}

impl Settings {
    pub fn new() -> Result<Settings> {
        let mut settings = Settings {
            height: Some(50),
            ..Default::default()
        };

        merge_if_exists(&mut settings, &util::get_config(CONF_PATH_COMPONENTS))?;
        merge_if_exists(&mut settings, &util::get_local(CONF_PATH_COMPONENTS))?;

        Ok(settings)
    }

    pub fn finder(&self) -> FinderChoice {
        self.finder.unwrap_or_default()
    }

    pub fn from_location(location: Location) -> Result<Settings> {
        let mut settings = Settings::default();
        match location {
            Location::Global => {
                merge_if_exists(&mut settings, &util::get_config(CONF_PATH_COMPONENTS))?
            }
            Location::Local => {
                merge_if_exists(&mut settings, &util::get_local(CONF_PATH_COMPONENTS))?
            }
        };
        Ok(settings)
    }

    pub fn filepath_from_location(location: Location) -> PathBuf {
        match location {
            Location::Global => util::get_config(CONF_PATH_COMPONENTS),
            Location::Local => util::get_local(CONF_PATH_COMPONENTS),
        }
    }

    pub fn write(&self, location: Location) -> Result<()> {
        let path = match location {
            Location::Global => util::get_config(CONF_PATH_COMPONENTS),
            Location::Local => util::get_local(CONF_PATH_COMPONENTS),
        };

        let contents = toml::to_string_pretty(self)?;
        util::write_content(path, &contents)?;

        Ok(())
    }

    pub fn contains_path(&self, path: &String) -> bool {
        self.workspace_paths.contains(path) || self.single_paths.contains(path)
    }

    pub fn add_path(&mut self, path: String, kind: PathKind) -> bool {
        match kind {
            PathKind::Single => self.single_paths.insert(path),
            PathKind::Workspace => self.workspace_paths.insert(path),
        }
    }

    pub fn list_paths(&self) -> HashSet<String> {
        let mut results = self
            .single_paths
            .iter()
            .map(|s| normalize_path(Path::new(&s)).display().to_string())
            .collect::<HashSet<_>>();

        let depth = self.depth.unwrap_or(10);
        for ws_path in &self.workspace_paths {
            let walker = WalkDir::new(ws_path)
                .skip_hidden(false)
                .max_depth(depth)
                .into_iter()
                .filter(|dir_entry_result| {
                    dir_entry_result
                        .as_ref()
                        .map(|dir_entry| {
                            if !dir_entry.file_type().is_dir() {
                                return false;
                            }

                            dir_entry
                                .file_name()
                                .to_str()
                                .map(|s| s == ".git" || s == ".bare")
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
                });

            for entry in walker {
                results.insert(entry.unwrap().parent_path().display().to_string());
            }
        }

        results
    }
}

fn merge_if_exists(settings: &mut Settings, path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let content = util::read_content(path)?;
    let raw: Settings = toml::from_str(&content)?;

    for p in raw.single_paths {
        settings.single_paths.insert(p);
    }

    for p in raw.workspace_paths {
        settings.workspace_paths.insert(p);
    }

    if raw.depth.is_some() {
        settings.depth = raw.depth;
    }

    if raw.height.is_some() {
        settings.height = raw.height.map(|v| v.clamp(1, 100))
    }

    if raw.finder.is_some() {
        settings.finder = raw.finder;
    }

    Ok(())
}

pub fn normalize_path<P: AsRef<Path> + ?Sized>(path: &P) -> Cow<Path> {
    let path = path.as_ref();
    if !path.starts_with("~") {
        return path.into();
    }

    HOME_DIR
        .join(
            path.strip_prefix("~")
                .expect("'~' was checked before stripping prefix"),
        )
        .into()
}
