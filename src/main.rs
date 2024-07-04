use clap::{Parser, Subcommand};
use lambda::{
    ast::from_cst::{from_tree, get_tree},
    runtime::eval,
    types::type_of,
};
use lsp::Backend;
use std::path::PathBuf;
use tower_lsp::{LspService, Server};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Lsp,
    Run { source: Option<PathBuf> },
    Debug { source: Option<PathBuf> },
}

mod lsp;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Command::Lsp => {
            let stdin = tokio::io::stdin();
            let stdout = tokio::io::stdout();

            let (service, socket) = LspService::new(Backend::new);
            Server::new(stdin, stdout, socket).serve(service).await;
        }
        Command::Debug { source } => {
            if let Some(source) = source {
                let source = std::fs::read_to_string(source).unwrap();
                let tree = get_tree(&source);
                println!("{:#}", tree.root_node());

                let (root, exprs) = from_tree(&tree, &source);
                let mut types = Default::default();

                println!("{:#?}", exprs.debug(root));
                match type_of(&exprs, &mut types, root) {
                    Err(e) => {
                        eprintln!("{e}");
                        return;
                    }
                    Ok(o) => {
                        println!(":: {:?}", o.debug(&types))
                    }
                }
            }
        }
        Command::Run { source } => {
            if let Some(source) = source {
                let source = std::fs::read_to_string(source).unwrap();
                let tree = get_tree(&source);
                let (root, exprs) = from_tree(&tree, &source);
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
    }
}

#[cfg(test)]
pub mod test_suite;

#[cfg(test)]
mod tests;
