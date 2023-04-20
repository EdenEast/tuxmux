use std::{
    io::Write,
    path::{Path, PathBuf},
};

use miette::{IntoDiagnostic, Result};

use crate::util;

#[derive(Debug, Default)]
pub struct Jumplist(pub Vec<String>);

impl Jumplist {
    pub fn path() -> PathBuf {
        util::get_local(&["jumplist"])
    }

    pub fn new() -> Result<Self> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Jumplist(vec![]));
        }

        let content = util::read_content(Self::path())?;
        Ok(Jumplist(
            content
                .lines()
                .filter(|e| Path::new(*e).exists())
                .map(|e| e.to_owned())
                .collect(),
        ))
    }

    pub fn add(&mut self, path: String) {
        if !self.0.contains(&path) {
            self.0.push(path);
        }
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.0.get(index).map(|x| x.as_str())
    }

    pub fn write(&self) -> Result<()> {
        util::write(Jumplist::path(), |f| {
            for e in &self.0 {
                f.write_fmt(format_args!("{}\n", e)).into_diagnostic()?;
            }
            Ok(())
        })?;

        Ok(())
    }
}
