use clap::{Command, CommandFactory};
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
    Generator,
};
use clap_mangen::Man;
use std::{env, fs::File, io::Error, path};

const BIN_NAME: &str = "tm";

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let out_dir = match env::var_os("OUT_DIR").map(PathBuf::from) {
        None => return Ok(()),
        Some(dir) => dir
            .ancestors()
            .nth(4)
            .expect("failed to determine out_dir")
            .to_owned(),
    };

    build_shell_completions(&out_dir)?;
    build_manpage(&out_dir)?;

    Ok(())
}

fn build_manpage(out_dir: &path::Path) -> Result<(), Error> {
    let man_dir = out_dir.join("man");
    std::fs::create_dir_all(&man_dir)?;
    let cli = Cli::command();
    let filename = man_dir.join(format!("{}.1", BIN_NAME));
    let mut file = File::create(&filename)?;
    Man::new(cli).render(&mut file)?;
    println!("cargo:info=manpage is generated: {:?}", &filename);
    Ok(())
}

fn build_shell_completions(out_dir: &path::Path) -> Result<(), Error> {
    let comp_dir = out_dir.join("completions");
    std::fs::create_dir_all(&comp_dir)?;
    let mut cli = Cli::command();
    generate_shell_completion(&mut cli, &comp_dir, Bash)?;
    generate_shell_completion(&mut cli, &comp_dir, Zsh)?;
    generate_shell_completion(&mut cli, &comp_dir, Fish)?;
    Ok(())
}

fn generate_shell_completion<T>(
    cmd: &mut Command,
    out_dir: &path::Path,
    shell: T,
) -> Result<PathBuf, Error>
where
    T: Generator,
{
    let path = generate_to(shell, cmd, BIN_NAME, &out_dir)?;
    println!("cargo:info=completion file is generated: {:?}", &path);
    Ok(path)
}
