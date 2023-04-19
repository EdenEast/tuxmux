mod attach;
mod cli;
mod jump;
mod kill;
mod list;
mod wcmd;

pub use crate::cmd::attach::use_cwd;
pub use crate::cmd::cli::*;

pub trait Run {
    fn run(self) -> eyre::Result<()>;
}
