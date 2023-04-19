use std::{env, path::Path};

use crate::{cmd::cli::Jump, jumplist::Jumplist, tmux};

use super::Run;

impl Run for Jump {
    fn run(self) -> eyre::Result<()> {
        if self.edit {
            let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_owned());
            std::process::Command::new(editor)
                .arg(Jumplist::path())
                .status()?;

            return Ok(());
        }

        let mut list = Jumplist::new()?;

        if self.list {
            for (i, elem) in list.0.iter().enumerate() {
                println!("{}: {}", i + 1, elem)
            }

            return Ok(());
        }

        if let Some(index) = self.index {
            if let Some(sel) = list.get(index.saturating_sub(1)) {
                let name = Path::new(sel).file_name().unwrap().to_str().unwrap();
                tmux::create_or_attach_session(name, sel)?;
            }

            return Ok(());
        }

        let path = match self.path {
            Some(path) => path,
            None => std::env::current_dir()?,
        };

        if !path.exists() {
            // TODO: Add logging with different types of sevarity
            return Err(eyre::eyre!("Invalid path: {}", path.display()));
        }

        list.add(path.canonicalize()?.display().to_string());
        list.write()?;

        Ok(())
    }
}
