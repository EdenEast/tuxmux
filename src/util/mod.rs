use eyre::Result;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

mod iter;

pub use iter::{intersperse, Intersperse};

pub fn get_config(components: &[&str]) -> PathBuf {
    let mut path = match (
        std::env::var("TM_CONFIG_PATH"),
        std::env::var("XDG_CONFIG_HOME"),
    ) {
        (Ok(p), _) => PathBuf::from_str(&p).unwrap(),
        (_, Ok(p)) => PathBuf::from_str(&p).unwrap().join("tm"),
        _ => dirs_next::config_dir().unwrap().join("tm"),
    };

    for c in components {
        path = path.join(c)
    }

    path
}

pub fn get_local(components: &[&str]) -> PathBuf {
    let mut path = match (
        std::env::var("TM_DATA_PATH"),
        std::env::var("XDG_DATA_HOME"),
    ) {
        (Ok(p), _) => PathBuf::from_str(&p).unwrap(),
        (_, Ok(p)) => PathBuf::from_str(&p).unwrap().join("tm"),
        _ => dirs_next::data_dir().unwrap().join("tm"),
    };

    for c in components {
        path = path.join(c)
    }

    path
}

pub fn read_content<P>(path: P) -> Result<String>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut content = String::new();
    std::fs::File::open(&path)?.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_content<P>(path: P, content: &str) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    std::fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn write<P, F>(path: P, write_fn: F) -> Result<()>
where
    P: AsRef<Path>,
    F: FnOnce(&mut File) -> Result<()>,
{
    std::fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    write_fn(&mut file)
}
