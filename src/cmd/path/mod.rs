use crate::cli::Path;

use super::ExecuteableCmd;

mod add;
mod list;
mod remove;

impl ExecuteableCmd for Path {
    fn execute(self) -> eyre::Result<()> {
        match self.cmd {
            crate::cli::PathCmd::Add(c) => c.execute(),
            crate::cli::PathCmd::List(c) => c.execute(),
            crate::cli::PathCmd::Remove(c) => c.execute(),
        }
    }
}
