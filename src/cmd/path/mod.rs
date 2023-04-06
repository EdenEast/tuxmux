use super::{Path, PathCmd, Run};

mod add;
mod list;
mod remove;

impl Run for Path {
    fn run(self) -> eyre::Result<()> {
        match self.cmd {
            PathCmd::Add(c) => c.run(),
            PathCmd::List(c) => c.run(),
            PathCmd::Remove(c) => c.run(),
        }
    }
}
