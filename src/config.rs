use std::{collections::HashSet, path::PathBuf};

use kdl::KdlError;
use thiserror::Error;

use crate::finder::FinderChoice;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    KdlError(#[from] KdlError),
    #[error("IoError: {0}")]
    Io(#[from]std::io::Error)
}

pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

pub struct Config {
    pub workspace: HashSet<PathBuf>,
    pub single: HashSet<PathBuf>,
    pub depth: usize,
    pub height: usize,
    pub finder: FinderChoice
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

// https://github.com/dj8yfo/meudeus/blob/master/src/config/mod.rs
// https://github.com/zellij-org/zellij/blob/main/zellij-utils/src/input/config.rs
// https://github.com/zellij-org/zellij/blob/main/zellij-utils/src/kdl/mod.rs
