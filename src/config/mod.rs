use crate::{finder::FinderChoice, util};
use indexmap::{indexmap, indexset, IndexMap, IndexSet};
use jwalk::WalkDir;

mod error;
mod parser;
mod source;

pub use error::ParseError;
pub use parser::Parser;
pub use source::Source;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Config {
    pub search: SearchPath,
    pub definitions: IndexMap<String, WorkspaceDefinition>,
    pub exclude_path: IndexSet<String>,
    pub depth: usize,
    pub height: usize,
    pub finder: FinderChoice,
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
            height: 50,
            finder: FinderChoice::default(),
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

    pub fn list_paths(&self) -> Vec<String> {
        let mut results: Vec<String> = self.search.single.clone();

        for ws_path in &self.search.workspace {
            let walker = WalkDir::new(ws_path)
                .skip_hidden(false)
                .max_depth(self.depth)
                .into_iter()
                .filter(|dir_entry_result| {
                    dir_entry_result
                        .as_ref()
                        .map(|dir_entry| {
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
                                dbg!(dir_entry);
                                return false;
                            }

                            let mut found = false;
                            for (_, def) in &self.definitions {
                                for file in &def.files {
                                    if dir_entry.path().join(file).exists() {
                                        found = true;
                                        break;
                                    }
                                }
                            }

                            found
                        })
                        .unwrap_or(false)
                });

            for entry in walker {
                results.push(entry.unwrap().path().display().to_string());
            }
        }

        results
    }
}
