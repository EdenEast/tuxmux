use std::{env, io::Result, path::PathBuf};

use clap_mangen::Man;

/// Man page can be created with:
/// `cargo run --bin tm-mangen`
/// in a directory specified by the environment variable OUT_DIR.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out_path = PathBuf::from(out_dir).join(format!("{}.1", env!("CARGO_PKG_NAME")));
    let cmd = tmgr::cmd::make_clap_command();
    let man = Man::new(cmd);
    let mut buffer = Vec::<u8>::new();
    man.render(&mut buffer)?;
    std::fs::write(&out_path, buffer)?;
    println!("Man page is generated at {:?}", out_path);
    Ok(())
}
