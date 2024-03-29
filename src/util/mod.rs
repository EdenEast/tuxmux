use miette::IntoDiagnostic;
use miette::Result;
use std::env::var;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

mod iter;

pub use iter::{intersperse, Intersperse};

pub fn get_editor() -> String {
    var("VISUAL").unwrap_or(var("EDITOR").unwrap_or("vi".to_string()))
}

pub fn get_config(components: &[&str]) -> PathBuf {
    let mut path = match (
        std::env::var("TUXMUX_CONFIG_PATH"),
        std::env::var("XDG_CONFIG_HOME"),
    ) {
        (Ok(p), _) => PathBuf::from_str(&p).unwrap(),
        (_, Ok(p)) => PathBuf::from_str(&p).unwrap().join("tuxmux"),
        _ => dirs_next::config_dir().unwrap().join("tuxmux"),
    };

    path.extend(components);
    path
}

pub fn format_name(name: &str) -> String {
    name.replace('.', "_")
}

pub fn get_local(components: &[&str]) -> PathBuf {
    let mut path = match (
        std::env::var("TUXMUX_DATA_PATH"),
        std::env::var("XDG_DATA_HOME"),
    ) {
        (Ok(p), _) => PathBuf::from_str(&p).unwrap(),
        (_, Ok(p)) => PathBuf::from_str(&p).unwrap().join("tuxmux"),
        _ => dirs_next::data_dir().unwrap().join("tuxmux"),
    };

    path.extend(components);
    path
}

pub fn read_content<P>(path: P) -> Result<String>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut content = String::new();
    std::fs::File::open(&path)
        .into_diagnostic()?
        .read_to_string(&mut content)
        .into_diagnostic()?;
    Ok(content)
}

pub fn write_content<P>(path: P, content: &str) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    std::fs::create_dir_all(path.as_ref().parent().unwrap()).into_diagnostic()?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .into_diagnostic()?;

    file.write_all(content.as_bytes()).into_diagnostic()?;

    Ok(())
}

pub fn write<P, F>(path: P, write_fn: F) -> Result<()>
where
    P: AsRef<Path>,
    F: FnOnce(&mut File) -> Result<()>,
{
    std::fs::create_dir_all(path.as_ref().parent().unwrap()).into_diagnostic()?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .into_diagnostic()?;

    write_fn(&mut file)
}
