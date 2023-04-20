use std::{collections::HashSet, fs::File, io::Read, path::PathBuf, str::FromStr};

use jwalk::WalkDir;
use kdl::KdlDocument;
use miette::{miette, IntoDiagnostic, Result};

use crate::{finder::FinderChoice, util};

const CONF_PATH_COMPONENTS: &[&str] = &["config.kdl"];

lazy_static::lazy_static! {
    static ref HOME_DIR: PathBuf = dirs_next::home_dir().unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Global,
    Local,
}

#[derive(Debug)]
pub struct Config {
    pub workspace: HashSet<PathBuf>,
    pub single: HashSet<PathBuf>,
    pub depth: usize,
    pub height: usize,
    pub finder: FinderChoice,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workspace: HashSet::new(),
            single: HashSet::new(),
            depth: 5,
            height: 40,
            finder: FinderChoice::default(),
        }
    }
}

macro_rules! kdl_first_entry_as_string_or_error {
    ( $node:expr, $error:expr ) => {
        $node
            .entries()
            .iter()
            .next()
            .and_then(|s| s.value().as_string())
            .ok_or(miette!($error))?
    };
}

macro_rules! kdl_first_entry_as_i64_or_error {
    ( $node:expr, $error:expr ) => {
        $node
            .entries()
            .iter()
            .next()
            .and_then(|s| s.value().as_i64())
            .ok_or(miette!($error))?
    };
}

impl Config {
    pub fn load() -> Result<Config> {
        let mut config = Config::default();

        let config_path = util::get_config(CONF_PATH_COMPONENTS);
        if config_path.exists() {
            config = Config::from_path(config_path, Some(config))?;
        }

        let local_path = util::get_local(CONF_PATH_COMPONENTS);
        if local_path.exists() {
            config = Config::from_path(local_path, Some(config))?;
        }

        Ok(config)
    }

    pub fn from_location(location: Location) -> Result<Config> {
        let mut config = Config::default();
        let path = match location {
            Location::Global => util::get_config(CONF_PATH_COMPONENTS),
            Location::Local => util::get_local(CONF_PATH_COMPONENTS),
        };

        if path.exists() {
            config = Config::from_path(path, Some(config))?;
        }

        Ok(config)
    }

    pub fn from_path<P>(path: P, default_config: Option<Config>) -> Result<Config>
    where
        P: std::convert::AsRef<std::path::Path>,
    {
        let mut file = File::open(path).into_diagnostic()?;
        let mut kdl_config = String::new();
        file.read_to_string(&mut kdl_config).into_diagnostic()?;
        Config::from_kdl(&kdl_config, default_config)
    }

    pub fn from_kdl(kdl_config: &str, base_config: Option<Config>) -> Result<Config> {
        let mut config = base_config.unwrap_or_default();
        let doc: KdlDocument = kdl_config.parse()?;

        if let Some(path_doc) = doc.get("paths").and_then(|p| p.children()) {
            for node in path_doc.nodes() {
                match node.name().value() {
                    "workspace" => {
                        let path =
                            kdl_first_entry_as_string_or_error!(node, "workspace requires value");
                        config.workspace.insert(to_path_buf(path));
                    }
                    "single" => {
                        let path =
                            kdl_first_entry_as_string_or_error!(node, "single requires value");
                        config.single.insert(to_path_buf(path));
                    }
                    c => {
                        return Err(miette!("unknown path type: {}", c));
                    }
                }
            }
        }

        if let Some(node) = doc.get("depth") {
            let value = kdl_first_entry_as_i64_or_error!(node, "depth requires number");
            config.depth = usize::try_from(value).unwrap_or(0);
        }

        if let Some(node) = doc.get("height") {
            let value = kdl_first_entry_as_i64_or_error!(node, "height requires number");
            config.height = usize::try_from(value).unwrap_or(0);
        }

        if let Some(node) = doc.get("finder") {
            let value = kdl_first_entry_as_string_or_error!(node, "finder requires value");
            config.finder = FinderChoice::from_str(&value)?;
        }

        Ok(config)
    }

    pub fn list_paths(&self) -> HashSet<String> {
        let mut results: HashSet<String> = self
            .single
            .iter()
            .map(|s| s.display().to_string())
            .collect();

        let depth = self.depth;
        for ws_path in &self.workspace {
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

fn to_path_buf(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        return HOME_DIR.join(
            path.strip_prefix("~/")
                .expect("'~/' was checked before stripping prefix"),
        );
    }

    return PathBuf::from(path);
}
