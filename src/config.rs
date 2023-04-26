use std::{collections::HashSet, path::Path, str::FromStr};

use jwalk::WalkDir;
use kdl::{KdlDocument, KdlError, KdlNode};
use miette::{miette, IntoDiagnostic, Result};

use crate::{finder::FinderChoice, util};

const CONF_PATH_COMPONENTS: &[&str] = &["config.kdl"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Global,
    Local,
}

#[derive(Debug)]
pub struct Config {
    pub workspace: HashSet<String>,
    pub single: HashSet<String>,
    pub exclude_path: Vec<String>,
    pub depth: usize,
    pub height: usize,
    pub finder: FinderChoice,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workspace: HashSet::from_iter(vec![shellexpand::tilde("~").to_string()].into_iter()),
            single: HashSet::new(),
            exclude_path: vec!["node_modules".to_string(), ".direnv".to_string()],
            depth: 5,
            height: 50,
            finder: FinderChoice::default(),
        }
    }
}

#[derive(Debug)]
struct Parser {
    source: String,
}

impl Parser {
    pub(crate) fn new(source: String) -> Self {
        Self { source }
    }

    pub(crate) fn from_file(path: &Path) -> Result<Self> {
        let source = std::fs::read_to_string(path).into_diagnostic()?;
        Ok(Self { source })
    }

    pub(crate) fn parse(self, default_config: Option<Config>) -> Result<Config> {
        let mut config = default_config.unwrap_or_default();
        let doc: KdlDocument = self.source.parse()?;

        if let Some(path_doc) = doc.get("paths") {
            //.and_then(|p| p.children()) {
            let default = match path_doc.get("default") {
                Some(value) => value.value().as_bool().ok_or(KdlError {
                    input: self.source.clone(),
                    span: *value.span(),
                    label: Some("invalid boolean"),
                    help: None,
                    kind: kdl::KdlErrorKind::Context("'default' requires a boolean value"),
                })?,
                None => true,
            };

            if !default {
                config.workspace = HashSet::new();
                config.single = HashSet::new();
            }

            for node in path_doc.children().map(|c| c.nodes()).unwrap_or_default() {
                match node.name().value() {
                    "workspace" => {
                        let path = self.first_entry_as_string(
                            node,
                            "path node `workspace` to contain a string value",
                        )?;
                        config
                            .workspace
                            .insert(shellexpand::tilde(path).to_string());
                    }
                    "single" => {
                        let path = self.first_entry_as_string(
                            node,
                            "path node `single` to contain a string value",
                        )?;
                        config.single.insert(shellexpand::tilde(path).to_string());
                    }
                    c => {
                        return Err(miette!("unknown path type: {}", c));
                    }
                }
            }
        }

        if let Some(exclude_doc) = doc.get("exclude_path") {
            let values = doc
                .get_dash_vals("exclude_path")
                .into_iter()
                .filter_map(|v| v.as_string().map(|s| s.to_string()))
                .collect::<Vec<_>>();

            let default = match exclude_doc.get("default") {
                Some(value) => value.value().as_bool().ok_or(KdlError {
                    input: self.source.clone(),
                    span: *value.span(),
                    label: Some("invalid boolean"),
                    help: None,
                    kind: kdl::KdlErrorKind::Context("'default' requires a boolean value"),
                })?,
                None => true,
            };

            if default {
                config.exclude_path.extend(values.into_iter());
            } else {
                config.exclude_path = values;
            }
        }

        if let Some(node) = doc.get("depth") {
            let value = self.first_entry_as_i64(node, "depth requires number")?;
            config.depth = usize::try_from(value).unwrap_or(0);
        }

        if let Some(node) = doc.get("height") {
            let value = self.first_entry_as_i64(node, "height requires number")?;
            config.height = usize::try_from(value).unwrap_or(0);
        }

        if let Some(node) = doc.get("finder") {
            let value = self.first_entry_as_string(node, "finder requires value")?;
            config.finder = FinderChoice::from_str(value)?;
        }

        Ok(config)
    }

    fn first_entry_as_string<'a>(
        &self,
        node: &'a KdlNode,
        msg: &'static str,
    ) -> std::result::Result<&'a str, KdlError> {
        node.entries()
            .iter()
            .next()
            .and_then(|s| s.value().as_string())
            .ok_or(KdlError {
                input: self.source.clone(),
                span: *node.span(),
                label: None,
                help: None,
                kind: kdl::KdlErrorKind::Context(msg),
            })
    }

    fn first_entry_as_i64(
        &self,
        node: &KdlNode,
        msg: &'static str,
    ) -> std::result::Result<i64, KdlError> {
        node.entries()
            .iter()
            .next()
            .and_then(|s| s.value().as_i64())
            .ok_or(KdlError {
                input: self.source.clone(),
                span: *node.span(),
                label: None,
                help: None,
                kind: kdl::KdlErrorKind::Context(msg),
            })
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        let mut config = Config::default();

        let config_path = util::get_config(CONF_PATH_COMPONENTS);
        if config_path.exists() {
            config = Parser::from_file(&config_path)?.parse(Some(config))?;
        }

        let local_path = util::get_local(CONF_PATH_COMPONENTS);
        if local_path.exists() {
            config = Parser::from_file(&local_path)?.parse(Some(config))?;
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
            config = Parser::from_file(&path)?.parse(Some(config))?;
        }

        Ok(config)
    }

    pub fn from_path<P>(path: P, default_config: Option<Config>) -> Result<Config>
    where
        P: std::convert::AsRef<std::path::Path>,
    {
        Parser::from_file(path.as_ref())?.parse(default_config)
    }

    pub fn from_kdl(kdl_config: &str, base_config: Option<Config>) -> Result<Config> {
        Parser::new(kdl_config.to_owned()).parse(base_config)
    }

    pub fn list_paths(&self) -> HashSet<String> {
        let mut results: HashSet<String> = self.single.clone();

        for ws_path in &self.workspace {
            let walker = WalkDir::new(ws_path)
                .skip_hidden(false)
                .max_depth(self.depth)
                .into_iter()
                .filter(|dir_entry_result| {
                    dir_entry_result
                        .as_ref()
                        .map(|dir_entry| {
                            if !dir_entry.file_type().is_dir() {
                                return false;
                            }

                            // Check if path is excluded
                            if dir_entry
                                .path()
                                .components()
                                .last()
                                .expect("always last component")
                                .as_os_str()
                                .to_str()
                                .map(|s| self.exclude_path.iter().any(|x| x == s))
                                .unwrap_or(true)
                            {
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

// TODO:
// https://github.com/orogene/orogene/blob/main/crates/oro-config/src/kdl_source.rs
