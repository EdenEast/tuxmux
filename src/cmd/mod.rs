pub mod attach;
pub mod config;
pub mod jump;
pub mod kill;
pub mod list;
pub mod path;
pub mod wcmd;

pub trait ExecuteableCmd {
    fn execute(self) -> eyre::Result<()>;
}
