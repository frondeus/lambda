use clap::{Parser, Subcommand};
use lambda::{
    ast::from_cst::{from_tree, get_tree},
    diagnostics::Diagnostics,
    runtime::eval,
    types::TypeEnv,
};
use lsp::Backend;
use std::path::PathBuf;
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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt().init();

    match args.command {
        Command::Lsp { stdin, stream } => {
            if stdin {
                let stdin = tokio::io::stdin();
                let stdout = tokio::io::stdout();

                let (service, socket) = LspService::new(Backend::new);
                return Server::new(stdin, stdout, socket).serve(service).await;
            }

            let stream = if stream {
                TcpStream::connect("127.0.0.1:9257").await.unwrap()
            } else {
                let listener = TcpListener::bind("127.0.0.1:9257").await.unwrap();
                let (stream, _) = listener.accept().await.unwrap();
                stream
            };

            let (read, write) = tokio::io::split(stream);
            let (service, socket) = LspService::new(Backend::new);
            Server::new(read, write, socket).serve(service).await;
        }
        Command::Debug { source } => {
            if let Some(source) = source {
                let source = std::fs::read_to_string(source).unwrap();
                let tree = get_tree(&source);
                println!("{:#}", tree.root_node());

                let (root, exprs) = from_tree(&tree, &source);
                let mut diagnostics = Diagnostics::default();
                let ir = lambda::ir::Exprs::from_ast(&exprs, root, &mut diagnostics);

                println!("{:#?}", exprs.debug(root));
                let (types, ty) = TypeEnv::infer(&ir, root, &mut diagnostics);
                // match TypeEnv::infer(&ir, root) {
                //     Err((_, e)) => {
                //         eprintln!("{e}");
                //         return;
                //     }
                // Ok((types, o)) => {
                println!(":: {:?}", ty.debug(&types))
                //     }
                // }
            }
        }
        Command::Run { source } => {
            if let Some(source_name) = source {
                let source = std::fs::read_to_string(source_name).unwrap();
                let tree = get_tree(&source);
                let (root, exprs) = from_tree(&tree, &source);
                let mut diagnostics = Diagnostics::default();
                let ir = lambda::ir::Exprs::from_ast(&exprs, root, &mut diagnostics);
                let mut runtime = Default::default();
                _ = TypeEnv::infer(&ir, root, &mut diagnostics);
                if diagnostics.has_errors() {
                    diagnostics.iter().for_each(|d| {
                        d.to_report()
                            .eprint(("test", ariadne::Source::from(source.clone())))
                            .unwrap();
                    });
                    return;
                }

                let result = eval(&exprs, &mut runtime, root);
                println!("{result}");
            }
        }
    }
}
