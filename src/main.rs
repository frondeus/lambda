#![deny(clippy::unwrap_used, clippy::expect_used)]

use clap::{Parser, Subcommand};
use lambda::{
    ast::from_cst::{from_tree, get_tree},
    diagnostics::Diagnostics,
    runtime::eval,
    types::TypeEnv,
};
use lsp::Backend;
use std::{path::PathBuf, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tower_lsp::{LspService, Server};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Lsp {
        #[arg(long)]
        stdin: bool,
        #[arg(long)]
        stream: bool,
    },
    Run {
        source: Option<PathBuf>,
    },
    Debug {
        source: Option<PathBuf>,
    },
}

mod lsp;

async fn main_inner() -> anyhow::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt().init();

    match args.command {
        Command::Lsp { stdin, stream } => {
            if stdin {
                let stdin = tokio::io::stdin();
                let stdout = tokio::io::stdout();

                let (service, socket) = LspService::new(Backend::new);
                Server::new(stdin, stdout, socket).serve(service).await;
                return Ok(());
            }

            let stream = if stream {
                TcpStream::connect("127.0.0.1:9257").await?
            } else {
                let listener = TcpListener::bind("127.0.0.1:9257").await?;
                let (stream, _) = listener.accept().await?;
                stream
            };

            let (read, write) = tokio::io::split(stream);
            let (service, socket) = LspService::new(Backend::new);
            Server::new(read, write, socket).serve(service).await;
        }
        Command::Debug { source } => {
            if let Some(source_name) = source {
                let source = tokio::fs::read_to_string(&source_name).await?;
                let tree = get_tree(&source);
                println!("{:#}", tree.root_node());

                let filename = source_name.display().to_string();
                let (root, exprs) = from_tree(&tree, &source, &filename);
                let Some(root) = root else {
                    eprintln!("<Nothing to do>");
                    return Ok(());
                };
                let mut diagnostics = Diagnostics::default();
                let ir = lambda::ir::Exprs::from_ast(&exprs, root, &mut diagnostics);

                println!("{:#?}", exprs.debug(Some(root)));
                let (types, ty) = TypeEnv::infer(&ir, root, &mut diagnostics);
                println!(":: {:?}", ty.debug(&types))
            }
        }
        Command::Run { source } => {
            if let Some(source_name) = source {
                let source = tokio::fs::read_to_string(&source_name).await?;
                let tree = get_tree(&source);
                let filename = source_name.display().to_string();
                let (root, exprs) = from_tree(&tree, &source, &filename);
                let Some(root) = root else {
                    eprintln!("<Nothing to do>");
                    return Ok(());
                };
                let mut diagnostics = Diagnostics::default();
                let ir = lambda::ir::Exprs::from_ast(&exprs, root, &mut diagnostics);
                let mut runtime = Default::default();
                _ = TypeEnv::infer(&ir, root, &mut diagnostics);
                if diagnostics.has_errors() {
                    for d in diagnostics.iter() {
                        d.to_report()
                            .eprint((Arc::from("test"), ariadne::Source::from(source.clone())))?;
                    }
                    return Ok(());
                }

                let result = eval(&exprs, &mut runtime, root);
                println!("{result}");
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = main_inner().await {
        eprintln!("{e:#}");
    }
}
