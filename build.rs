use clap::{Command, CommandFactory};
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
    Generator,
};
use std::{
    env,
    fs::File,
    io::{Error, Write},
    path,
};

const BIN_NAME: &str = "tux";

include!("src/cmd/cli.rs");

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

    build_shell_completions(&out_dir.join("completions"))?;
    build_manpages(&out_dir.join("man"))?;

    Ok(())
}

fn build_manpages(out_dir: &path::Path) -> Result<(), Error> {
    std::fs::create_dir_all(out_dir)?;
    fn build(dir: &path::Path, app: &Command) -> Result<(), Error> {
        // `get_display_name()` is `Some` for all instances, except the root.
        let name = app.get_bin_name().unwrap_or_else(|| app.get_name());
        let mut out = File::create(dir.join(format!("{name}.1")))?;

        clap_mangen::Man::new(app.clone()).render(&mut out)?;
        out.flush()?;

        for sub in app.get_subcommands() {
            build(dir, sub)?;
        }

        Ok(())
    }

    build(out_dir, &Cli::command())
}

fn build_shell_completions(out_dir: &path::Path) -> Result<(), Error> {
    std::fs::create_dir_all(out_dir)?;
    let mut cli = Cli::command();
    generate_shell_completion(&mut cli, out_dir, Bash)?;
    generate_shell_completion(&mut cli, out_dir, Zsh)?;
    generate_shell_completion(&mut cli, out_dir, Fish)?;
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
    let path = generate_to(shell, cmd, BIN_NAME, out_dir)?;
    println!("cargo:info=completion file is generated: {:?}", &path);
    Ok(path)
}
