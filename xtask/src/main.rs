use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

mod clippy;
mod fmt;
mod gen_syntax;
mod repl;
mod review_tests;
mod test;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(-1);
    }
}

type DynError = Box<dyn std::error::Error>;
type Result<T = (), E = DynError> = std::result::Result<T, E>;

fn try_main() -> Result {
    let task = env::args().nth(1);
    let root = project_root();
    match task.as_deref() {
        Some("gen-syntax") => gen_syntax::run(&root)?,
        Some("review-tests") => review_tests::run(&root)?,
        Some("test") => test::run(&root)?,
        Some("repl") => repl::run(&root)?,
        Some("clippy") => clippy::run(&root)?,
        Some("fmt") => fmt::run(&root)?,
        Some("ci") => {
            fmt::run(&root)?;
            clippy::run(&root)?;
            gen_syntax::run(&root)?;
            let res = test::run(&root);
            review_tests::run(&root)?;
            res?;
        }
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
        gen-syntax - Generate TreeSitter parser
        test - Run all tests (including TreeSitter tests)
        review-tests - Review snapshot tests

        repl - Run REPL

        ci - ['gen-syntax', 'test', 'review-tests']
    "
    );
}

pub fn project_root() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

pub fn run_command(desc: &str, dir: impl AsRef<Path>, cmd: &str, args: &[&str]) -> Result {
    let status = Command::new(cmd).current_dir(dir).args(args).status()?;

    if !status.success() {
        Err(format!("{desc} failed"))?;
    }
    Ok(())
}
