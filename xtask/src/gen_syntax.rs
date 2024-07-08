use std::path::Path;

use crate::{run_command, Result};

pub fn run(root: &Path) -> Result {
    run_command(
        "Tree Sitter codegen",
        root.join("cst"),
        "tree-sitter",
        &["generate"],
    )?;

    println!("Generated syntax");
    Ok(())
}
