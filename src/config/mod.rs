use crate::util;
use indexmap::{indexmap, indexset, IndexMap, IndexSet};

mod error;
mod parser;
mod source;

pub use error::ParseError;
pub use parser::Parser;
pub use source::Source;

#[derive(Debug, Clone)]
pub struct WorkspaceDefinition {
    pub name: String,
    pub files: Vec<String>,
    pub layout: Option<String>,
}

#[derive(Debug)]
pub struct SearchPath {
    pub workspace: Vec<String>,
    pub single: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Mode {
    Full,
    Inline(u8),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Inline(50)
    }
}

#[derive(Debug)]
pub struct Config {
    pub search: SearchPath,
    pub definitions: IndexMap<String, WorkspaceDefinition>,
    pub exclude_path: IndexSet<String>,
    pub depth: usize,
    pub mode: Mode,
}

impl Default for WorkspaceDefinition {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            files: vec![".git".to_string(), ".bare".to_string()],
            layout: None,
        }
    }
}

impl Default for SearchPath {
    fn default() -> Self {
        Self {
            workspace: vec![shellexpand::tilde("~").to_string()],
            single: vec![],
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search: SearchPath::default(),
            exclude_path: indexset! { "node_modules".to_string(), ".direnv".to_string() },
            depth: 5,
            mode: Mode::default(),
            definitions: indexmap! {"default".to_string() => WorkspaceDefinition::default()},
        }
    }
}

impl Config {
    pub fn load() -> Result<Config, ParseError> {
        let mut config = Config::default();

        let config_path = util::get_config(&["config.kdl"]);
        if config_path.exists() {
            config = Parser::new(Source::load(config_path.display().to_string())?)
                .parse_with_default(Some(config))?;
        }

        let local_path = util::get_local(&["config.kdl"]);
        if local_path.exists() {
            config = Parser::new(Source::load(local_path.display().to_string())?)
                .parse_with_default(Some(config))?;
        }

        Ok(config)
    }
}
