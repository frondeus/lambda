use super::*;

pub fn run(root: &Path) -> Result {
    run_command("REPL", root, "cargo", &["fmt", "--all"])
}
