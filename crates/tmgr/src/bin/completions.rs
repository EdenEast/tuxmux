use clap::ArgEnum;
use clap_complete::Shell;
use std::env;
use std::io::Result;

/// Shell completions can be created with:
/// `cargo run --bin tm-completions`
/// in a directory specified by the environment variable OUT_DIR.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let mut cmd = tmgr::cmd::make_clap_command();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut cmd, env!("CARGO_PKG_NAME"), &out_dir)?;
    }
    println!("Completion scripts are generated in {:?}", out_dir);
    Ok(())
}
