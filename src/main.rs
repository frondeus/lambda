use clap::Parser;
use lambda::{ast::from_cst::from_source, runtime::eval, types::type_of};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    source: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if let Some(source) = args.source {
        let source = std::fs::read_to_string(source).unwrap();
        let (root, exprs) = from_source(&source);
        let mut types = Default::default();
        let mut runtime = Default::default();
        if let Err(e) = type_of(&exprs, &mut types, root) {
            eprintln!("{e}");
            return;
        }

        let result = eval(&exprs, &mut runtime, root);
        println!("{result}");
    }
}

#[cfg(test)]
pub mod test_suite;

#[cfg(test)]
mod tests;
