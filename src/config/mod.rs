use crate::{mux::Mux, util};
use indexmap::{indexset, IndexSet};

mod error;
mod parser;
mod source;

pub use error::ParseError;
pub use parser::Parser;
pub use source::Source;

#[derive(Debug)]
pub struct SearchPath {
    pub workspace: Vec<String>,
    pub single: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Full,
    Lines(u16),
    Percentage(f32),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Percentage(0.5)
    }
}

#[derive(Debug)]
pub struct Config {
    pub search: SearchPath,
    pub exclude_path: IndexSet<String>,
    pub depth: usize,
    pub mode: Mode,
    pub default_worktree: bool,
    pub mux: Mux,
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
            exclude_path: indexset! { "node_modules".to_string(), ".direnv".to_string(), ".cache".to_string(), ".local".to_string()},
            depth: 5,
            mode: Mode::default(),
            default_worktree: false,
            mux: Mux::default(),
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
