use crate::{run_command, Result};
use std::path::Path;

pub fn run(root: &Path) -> Result {
    run_command(
        "Create new session",
        root,
        "zellij",
        &["-l", "./zellij-layout.kdl", "a", "lambda", "-c"],
    )?;

    Ok(())
}
