use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
    sync::Arc,
};

use lambda::{
    ast::{
        from_cst::{from_tree, get_tree, get_tree_diff, to_spanned},
        queries::Queries,
        SyntaxTree,
    },
    diagnostics::Diagnostics,
    types::TypeEnv,
};
use ropey::Rope;
use tokio::sync::RwLock;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams,
        CompletionResponse, Diagnostic, DiagnosticSeverity, DidChangeConfigurationParams,
        DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidChangeWorkspaceFoldersParams,
        DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
        HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams, InlayHint,
        InlayHintKind, InlayHintLabel, InlayHintParams, Location, MarkedString, MessageType, OneOf,
        ReferenceParams, RenameParams, ServerCapabilities, TextDocumentContentChangeEvent,
        TextDocumentPositionParams, TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit,
        Url, WorkspaceEdit, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
    },
    Client, LanguageServer,
};
use tree_sitter::Point;
use utils::{intersects, to_point, to_position, NodeExt, RopeExt};

mod utils;

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
    filename: String,
    tree: SyntaxTree,
    source: Rope,
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

    async fn file_from_text_document_position(
        &self,
        params: &TextDocumentPositionParams,
    ) -> Option<(Arc<File>, Point)> {
        let uri = params.text_document.uri.to_file_path().ok();
        let file = self.state.read().await.maybe_get_tree(uri)?;

        let position = &params.position;
        let point = Point::new(position.line as usize, position.character as usize);
        Some((file, point))
    }

    async fn update_file_with_changes(
        &self,
        file_path: PathBuf,
        changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Option<Arc<File>> {
        let mut state = self.state.write().await;
        let old_state = state.forest.get_mut(&file_path)?;

        let mut source = old_state.source.clone();
        let mut tree = old_state.tree.clone();
        let mut offset: i64 = 0;
        for TextDocumentContentChangeEvent {
            range,
            range_length: _,
            text,
        } in changes
        {
            match range {
                None => {
                    source = Rope::from_str(&text);
                }
                Some(range) => {
                    let start_offset =
                        (old_state.source.to_byte(range.start) as i64 - offset) as usize;
                    let end_offset = (old_state.source.to_byte(range.end) as i64 - offset) as usize;

                    // tracing::info!("Change: {start_offset}..{end_offset} ({offset}) `{text}`");
                    let old_len = end_offset - start_offset;
                    let new_len = text.len();
                    source.remove(start_offset..end_offset);
                    source.insert(start_offset, &text);
                    offset = old_len as i64 - new_len as i64;
                    // tracing::info!("Change offset: {old_len} - {new_len} = {offset}");
                    let edit = old_state.source.to_input_edit(range, &text);
                    tree.edit(&edit);
                }
            }
        }

        let src = format!("{source}");
        let new_tree = get_tree_diff(&src, &tree);
        // tracing::info!("New tree: {:#}", new_tree.root_node());
        *old_state = Arc::new(File {
            source,
            tree: new_tree,
            filename: old_state.filename.clone(),
        });
        Some(Arc::clone(old_state))
    }

    async fn update_file(&self, file_path: PathBuf) -> Option<Arc<File>> {
        let Ok(source) = tokio::fs::read_to_string(&file_path).await else {
            return None;
        };
        let source = Rope::from_str(&source);
        let filename = file_path.display().to_string();

        let mut state = self.state.write().await;

        match state.forest.entry(file_path) {
            Entry::Occupied(mut slot) => {
                let old_file = slot.get();
                let old_tree = &old_file.tree;
                let new_tree = get_tree_diff(&source.to_string(), old_tree);
                let arc = Arc::new(File {
                    tree: new_tree,
                    source,
                    filename,
                });
                slot.insert(Arc::clone(&arc));
                Some(arc)
            }
            Entry::Vacant(slot) => {
                let arc = Arc::new(File {
                    tree: get_tree(&source.to_string()),
                    source,
                    filename,
                });
                slot.insert(Arc::clone(&arc));
                Some(arc)
            }
        }
    }

    async fn publish_diagnostics(&self, file: Option<Arc<File>>, uri: Url) {
        let Some(file) = file else {
            return;
        };
        let File {
            tree,
            source,
            filename,
        } = &*file;
        let src = format!("{source}");
        let (root_expr, exprs) = from_tree(tree, &src, filename);
        let mut diagnostics = Diagnostics::default();
        let Some(root_expr) = root_expr else {
            return;
        };
        let ir = lambda::ir::Exprs::from_ast(&exprs, root_expr, &mut diagnostics);
        _ = TypeEnv::infer(&ir, root_expr, &mut diagnostics);

        let diagnostics = diagnostics
            .iter()
            .map(|i| Diagnostic {
                range: source.to_lsp_range(i.message.range),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("lambda".to_string()),
                message: i.message.node.clone(),
                related_information: None,
                tags: None,
                data: None,
            })
            .collect();
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
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
                references_provider: Some(OneOf::Left(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![" ".to_string()]),
                    all_commit_characters: None,
                    work_done_progress_options: Default::default(),
                    completion_item: Default::default(),
                }),
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
        let file = self.update_file(file_path).await;
        self.publish_diagnostics(file, param.text_document.uri)
            .await;
    }

    async fn did_change(&self, param: DidChangeTextDocumentParams) {
        let Ok(file_path) = param.text_document.uri.to_file_path() else {
            return;
        };
        tracing::info!("Did change {:?}", file_path);
        // self.update_file(file_path).await;
        let file = self
            .update_file_with_changes(file_path, param.content_changes)
            .await;

        self.publish_diagnostics(file, param.text_document.uri)
            .await;
    }

    async fn did_save(&self, param: DidSaveTextDocumentParams) {
        let Ok(file_path) = param.text_document.uri.to_file_path() else {
            return;
        };
        tracing::info!("Did save {:?}", file_path);
        let file = self.update_file(file_path).await;
        self.publish_diagnostics(file, param.text_document.uri)
            .await;
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
        Ok(self.hover_inner(params).await)
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let res = self.inlay_hint_inner(params).await.unwrap_or_default();

        Ok(Some(res))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(self.goto_definition_inner(params).await)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(self.completion_inner(params).await)
    }

    async fn completion_resolve(&self, params: CompletionItem) -> Result<CompletionItem> {
        Ok(params)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let res = self.references_inner(params).await.unwrap_or_default();
        Ok(Some(res))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        Ok(self.rename_inner(params).await)
    }
}

impl Backend {
    async fn hover_inner(&self, params: HoverParams) -> Option<Hover> {
        let (file, point) = self
            .file_from_text_document_position(&params.text_document_position_params)
            .await?;
        let File {
            tree,
            source,
            filename,
        } = &*file;

        let src = format!("{source}");
        let (root_expr, exprs) = from_tree(tree, &src, filename);
        let root_expr = root_expr?;
        let mut diagnostics = Diagnostics::default();
        let ir = lambda::ir::Exprs::from_ast(&exprs, root_expr, &mut diagnostics);

        let root = tree.root_node();
        let node = root.named_descendant_for_point_range(point, point)?;
        let node = to_spanned(node, &src, filename);

        let node_expr_id = exprs.find_expr_with_node(node)?;

        let (types, _ty) = TypeEnv::infer(&ir, root_expr, &mut diagnostics);

        let mut markdown = String::new();

        let ty = types.type_of(node_expr_id)?;
        markdown += &format!("\n\n```\n{}\n```", ty.debug(&types));

        Some(Hover {
            contents: HoverContents::Scalar(MarkedString::from_markdown(markdown)),
            range: None,
        })
    }

    async fn inlay_hint_inner(&self, params: InlayHintParams) -> Option<Vec<InlayHint>> {
        let file = self
            .state
            .read()
            .await
            .maybe_get_tree(params.text_document.uri.to_file_path().ok())?;
        let File {
            tree,
            source,
            filename,
        } = &*file;

        let src = format!("{source}");
        let (root_expr, exprs) = from_tree(tree, &src, filename);
        let root_expr = root_expr?;
        let mut diagnostics = Diagnostics::default();
        let ir = lambda::ir::Exprs::from_ast(&exprs, root_expr, &mut diagnostics);
        let (types, _) = TypeEnv::infer(&ir, root_expr, &mut diagnostics);

        let range = params.range;
        let range_start = to_point(range.start);
        let range_end = to_point(range.end);
        let mut hints = Vec::new();
        for (e, ty) in types.exprs() {
            let e = exprs.get(e);
            let Some(node) = e.node() else {
                continue;
            };
            if e.is_literal() {
                continue;
            }
            let sp = node.node.start_position();
            let ep = node.node.end_position();
            let ty = ty.debug(&types);

            if intersects((range_start, range_end), (sp, ep)) {
                hints.push(InlayHint {
                    position: to_position(ep),
                    label: InlayHintLabel::String(format!("{}", ty)),
                    kind: Some(InlayHintKind::TYPE),
                    text_edits: None,
                    data: None,
                    padding_left: Some(true),
                    padding_right: None,
                    tooltip: None,
                });
            }
        }

        Some(hints)
    }

    async fn goto_definition_inner(
        &self,
        params: GotoDefinitionParams,
    ) -> Option<GotoDefinitionResponse> {
        let (file, point) = self
            .file_from_text_document_position(&params.text_document_position_params)
            .await?;
        let File {
            tree,
            source,
            filename,
        } = &*file;
        let src = format!("{source}");

        let (root_expr, exprs) = from_tree(tree, &src, filename);
        let root_expr = root_expr?;

        let mut diagnostics = Diagnostics::default();
        let ir = lambda::ir::Exprs::from_ast(&exprs, root_expr, &mut diagnostics);

        let root = tree.root_node();
        let node = root.named_descendant_for_point_range(point, point)?;
        let node = to_spanned(node, &src, filename);
        let node_expr_id = exprs.find_expr_with_node(node)?;

        let expr = ir.get(node_expr_id);
        let var = match expr {
            lambda::ir::Expr::Var {
                name: _,
                id: Some(id),
                node: _,
            } => *id,
            _ => return None,
        };
        let var = ir.get_var(var);
        let def_id = var.defined;
        let def = ir.get(def_id).node()?;

        let def_range = NodeExt::range(&def);

        Some(GotoDefinitionResponse::Scalar(Location {
            // For now we have only one file
            uri: params.text_document_position_params.text_document.uri,
            range: def_range,
        }))
    }

    async fn completion_inner(&self, params: CompletionParams) -> Option<CompletionResponse> {
        let (file, point) = self
            .file_from_text_document_position(&params.text_document_position)
            .await?;
        let File {
            tree,
            source,
            filename,
        } = &*file;
        let src = source.to_string();
        let (root_expr, exprs) = from_tree(tree, &src, filename);
        let root_expr = root_expr?;

        let mut diagnostics = Diagnostics::default();
        let ir = lambda::ir::Exprs::from_ast(&exprs, root_expr, &mut diagnostics);
        let (types, _) = TypeEnv::infer(&ir, root_expr, &mut diagnostics);

        let mut scopes = ir.scopes_in_point(point).collect::<Vec<_>>();
        scopes.sort_by_key(|s| s.depth);

        let completions = scopes
            .into_iter()
            .flat_map(|scope| {
                scope.vars.iter().map(|(var_name, var_id)| {
                    let def = ir.get_var(*var_id);
                    let ty = types.type_of(def.defined).map(|ty| types.print_type(ty));

                    CompletionItem {
                        label: exprs.get_str(*var_name).to_string(),
                        label_details: None,
                        kind: Some(CompletionItemKind::VARIABLE),
                        detail: ty,
                        documentation: None,
                        deprecated: None,
                        preselect: None,
                        sort_text: None,
                        filter_text: None,
                        insert_text: None,
                        insert_text_format: None,
                        insert_text_mode: None,
                        text_edit: None,
                        additional_text_edits: None,
                        command: None,
                        commit_characters: None,
                        data: None,
                        tags: None,
                    }
                })
            })
            .collect();

        Some(CompletionResponse::Array(completions))
    }

    async fn references_inner(&self, params: ReferenceParams) -> Option<Vec<Location>> {
        let (file, point) = self
            .file_from_text_document_position(&params.text_document_position)
            .await?;
        let File {
            tree,
            source,
            filename,
        } = &*file;
        let src = format!("{source}");

        let (root_expr, exprs) = from_tree(tree, &src, filename);
        let root_expr = root_expr?;

        let mut diagnostics = Diagnostics::default();
        let ir = lambda::ir::Exprs::from_ast(&exprs, root_expr, &mut diagnostics);

        let root = tree.root_node();
        let node = root.named_descendant_for_point_range(point, point)?;
        let node = to_spanned(node, &src, filename);
        let node_expr_id = exprs.find_expr_with_node(node)?;

        let expr = ir.get(node_expr_id);
        let var = match expr {
            lambda::ir::Expr::Var {
                name: _,
                id: Some(id),
                node: _,
            } => *id,
            lambda::ir::Expr::VarDef {
                name: _,
                id,
                node: _,
            } => *id,
            _ => return None,
        };
        let var = ir.get_var(var);
        let ref_ids = &var.references;

        let refs = ref_ids
            .iter()
            .filter_map(|id| ir.get(*id).node())
            .map(|node| NodeExt::range(&node))
            .map(|range| Location {
                uri: params.text_document_position.text_document.uri.clone(),
                range,
            })
            .collect::<Vec<_>>();

        Some(refs)
    }

    async fn rename_inner(&self, params: RenameParams) -> Option<WorkspaceEdit> {
        let (file, point) = self
            .file_from_text_document_position(&params.text_document_position)
            .await?;
        let File {
            tree,
            source,
            filename,
        } = &*file;
        let src = format!("{source}");

        let (root_expr, exprs) = from_tree(tree, &src, filename);
        let root_expr = root_expr?;

        let mut diagnostics = Diagnostics::default();
        let ir = lambda::ir::Exprs::from_ast(&exprs, root_expr, &mut diagnostics);

        let root = tree.root_node();
        let node = root.named_descendant_for_point_range(point, point)?;
        let node = to_spanned(node, &src, filename);
        let node_expr_id = exprs.find_expr_with_node(node)?;
        let expr = ir.get(node_expr_id);
        let var = match expr {
            lambda::ir::Expr::Var {
                name: _,
                id: Some(id),
                node: _,
            } => *id,
            lambda::ir::Expr::VarDef {
                name: _,
                id,
                node: _,
            } => *id,
            _ => return None,
        };
        let var = ir.get_var(var);

        let text_edits = var
            .all_occurrences()
            .filter_map(|e| ir.get(e).node())
            .map(|node| TextEdit {
                range: NodeExt::range(&node),
                new_text: params.new_name.clone(),
            })
            .collect();

        let changes =
            std::iter::once((params.text_document_position.text_document.uri, text_edits))
                .collect();
        Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        })
    }
}
