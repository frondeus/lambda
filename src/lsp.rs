use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
    sync::Arc,
};

use lambda::{
    ast::{
        from_cst::{from_tree, get_tree, get_tree_diff},
        queries::Queries,
        SyntaxTree,
    },
    types::{type_of, TypeEnv},
};
use tokio::sync::RwLock;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        DidChangeConfigurationParams, DidChangeTextDocumentParams, DidChangeWatchedFilesParams,
        DidChangeWorkspaceFoldersParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
        DidSaveTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse, Hover,
        HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult,
        InitializedParams, InlayHint, InlayHintKind, InlayHintLabel, InlayHintLabelPart,
        InlayHintParams, Location, MarkedString, MessageType, OneOf, Position, Range,
        ServerCapabilities, TextDocumentContentChangeEvent, TextDocumentSyncCapability,
        TextDocumentSyncKind, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
    },
    Client, LanguageServer,
};
use tree_sitter::Point;

#[derive(Debug)]
pub struct Backend {
    client: Client,
    state: RwLock<State>,
}

#[derive(Debug, Default)]
pub struct State {
    /// Forest is... a collection of tree sitter trees :)
    forest: HashMap<PathBuf, Arc<File>>,
}

#[derive(Debug)]
struct File {
    tree: SyntaxTree,
    source: String,
}

impl State {
    fn get_tree(&self, p: impl AsRef<Path>) -> Option<Arc<File>> {
        let p = p.as_ref().to_owned();
        self.forest.get(&p).map(Arc::clone)
    }

    fn maybe_get_tree(&self, p: Option<PathBuf>) -> Option<Arc<File>> {
        let p = p?;
        self.get_tree(p)
    }
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            state: Default::default(),
        }
    }

    async fn update_file_with_changes(
        &self,
        file_path: PathBuf,
        changes: Vec<TextDocumentContentChangeEvent>,
    ) {
        let mut state = self.state.write().await;
        let Some(old_state) = state.forest.get_mut(&file_path) else {
            return;
        };

        let mut source = old_state.source.clone();
        for TextDocumentContentChangeEvent {
            range,
            range_length: _,
            text,
        } in changes
        {
            match range {
                None => {
                    source = text;
                }
                Some(Range { start, end }) => {
                    let start = position_to_offset(start, &old_state.source);
                    let end = position_to_offset(end, &old_state.source);

                    source.replace_range(start..end, &text);
                }
            }
        }
        // tracing::info!("After: {source}");

        let new_tree = get_tree_diff(&source, &old_state.tree);
        *old_state = Arc::new(File {
            source,
            tree: new_tree,
        });
    }

    async fn update_file(&self, file_path: PathBuf) {
        let Ok(source) = tokio::fs::read_to_string(&file_path).await else {
            return;
        };

        let mut state = self.state.write().await;

        match state.forest.entry(file_path) {
            Entry::Occupied(mut slot) => {
                let old_file = slot.get();
                let old_tree = &old_file.tree;
                let new_tree = get_tree_diff(&source, old_tree);
                slot.insert(Arc::new(File {
                    tree: new_tree,
                    source,
                }));
            }
            Entry::Vacant(slot) => {
                slot.insert(Arc::new(File {
                    tree: get_tree(&source),
                    source,
                }));
            }
        };
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("Initialized");
    }

    async fn did_open(&self, param: DidOpenTextDocumentParams) {
        let Ok(file_path) = param.text_document.uri.to_file_path() else {
            return;
        };
        tracing::info!("Did open {:?}", file_path);
        self.update_file(file_path).await;
    }

    async fn did_change(&self, param: DidChangeTextDocumentParams) {
        let Ok(file_path) = param.text_document.uri.to_file_path() else {
            return;
        };
        tracing::info!("Did change {:?}", file_path);
        // self.update_file(file_path).await;
        self.update_file_with_changes(file_path, param.content_changes).await;
    }

    async fn did_save(&self, param: DidSaveTextDocumentParams) {
        let Ok(file_path) = param.text_document.uri.to_file_path() else {
            return;
        };
        tracing::info!("Did save {:?}", file_path);
        self.update_file(file_path).await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        tracing::info!("Did close");
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        tracing::info!("Did change workspace folders");
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        tracing::info!("Did change configuration");
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        tracing::info!("Did change watched files");
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let Some(file) = self.state.read().await.maybe_get_tree(
            params
                .text_document_position_params
                .text_document
                .uri
                .to_file_path()
                .ok(),
        ) else {
            return Ok(None);
        };
        let File { tree, source } = &*file;

        let position = &params.text_document_position_params.position;
        let point = Point::new(position.line as usize, position.character as usize);

        let (root_expr, exprs) = from_tree(tree, source);
        let mut types = TypeEnv::default();

        let root = tree.root_node();
        let node = root.named_descendant_for_point_range(point, point).unwrap();

        let Some(node_expr_id) = exprs.find_expr_with_node(node) else {
            return Ok(None);
        };

        let ty = type_of(&exprs, &mut types, root_expr);

        let mut markdown = String::new();

        match ty {
            Ok(_) => {
                let ty = types.type_of(node_expr_id).unwrap();
                markdown += &format!("\n\n```\n{}\n```", ty.debug(&types));
            }
            Err(e) => {
                markdown += &format!("\n\n{e}");
            }
        }

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::from_markdown(markdown)),
            range: None,
        }))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let Some(file) = self
            .state
            .read()
            .await
            .maybe_get_tree(params.text_document.uri.to_file_path().ok())
        else {
            return Ok(None);
        };
        let File { tree, source } = &*file;

        let (root_expr, exprs) = from_tree(tree, source);
        let mut types = TypeEnv::default();

        // let root = tree.root_node();
        _ = type_of(&exprs, &mut types, root_expr);
        let range = params.range;
        let range_start = position_to_point(range.start);
        let range_end = position_to_point(range.end);
        let mut hints = Vec::new();
        for (e, ty) in types.exprs() {
            let e = exprs.get(e);
            let Some(node) = e.node() else {
                continue;
            };
            if e.is_literal() {
                continue;
            }
            let sp = node.start_position();
            let ep = node.end_position();
            let ty = ty.debug(&types);

            if intersects((range_start, range_end), (sp, ep)) {
                hints.push(InlayHint {
                    position: point_to_position(ep),
                    // label: InlayHintLabel::String(format!("{}", ty)),
                    label: InlayHintLabel::LabelParts(vec![InlayHintLabelPart {
                        value: format!("{}", ty),
                        tooltip: None,
                        location: Some(Location {
                            uri: params.text_document.uri.clone(),
                            range: Range {
                                start: Position::new(0, 4),
                                end: Position::new(0, 5),
                            },
                        }),
                        command: None,
                    }]),
                    kind: Some(InlayHintKind::TYPE),
                    text_edits: None,
                    data: None,
                    padding_left: Some(true),
                    padding_right: None,
                    tooltip: None,
                });
            }
        }

        // tracing::info!("{:#}", tree.root_node());
        tracing::info!("Hints: {}", hints.len());

        Ok(Some(hints))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        tracing::info!("GOTO: {params:#?}");
        Ok(None)
    }
}

fn point_to_position(point: Point) -> Position {
    Position {
        line: point.row as u32,
        character: point.column as u32,
    }
}

fn position_to_point(pos: Position) -> Point {
    Point::new(pos.line as usize, pos.character as usize)
}

fn intersects(a: (Point, Point), b: (Point, Point)) -> bool {
    let (a_start, a_end) = a;
    let (b_start, b_end) = b;
    a_start.row <= b_end.row
        && a_end.row >= b_start.row
        && a_start.column <= b_end.column
        && a_end.column >= b_start.column
}

fn position_to_offset(pos: Position, source: &str) -> usize {
    let Position { line, character } = pos;

    // Based on line & character find offset in source
    source
        .lines()
        .take(line as usize)
        .map(|l| l.len() + 1)
        .sum::<usize>()
        + character as usize
}
