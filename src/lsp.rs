use lambda::{
    ast::{
        from_cst::{from_tree, get_tree},
        queries::Queries,
    },
    types::{type_of, TypeEnv},
};
use tower_lsp::{
    jsonrpc::{Error, ErrorCode, Result},
    lsp_types::{
        GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
        HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams, InlayHint,
        InlayHintLabel, InlayHintParams, MarkedString, MessageType, OneOf, Position,
        ServerCapabilities,
    },
    Client, LanguageServer,
};
use tree_sitter::Point;

#[derive(Debug)]
pub struct Backend {
    client: Client,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let file_uri = &params.text_document_position_params.text_document.uri;
        let Ok(file_path) = file_uri.to_file_path() else {
            return Ok(None);
        };
        let file = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|_e| Error::new(ErrorCode::InternalError))?;

        let position = &params.text_document_position_params.position;
        let point = Point::new(position.line as usize, position.character as usize);

        let tree = get_tree(&file);
        let (root_expr, exprs) = from_tree(&tree, &file);
        let mut types = TypeEnv::default();

        let root = tree.root_node();
        let node = root.named_descendant_for_point_range(point, point).unwrap();

        let Some(node_expr_id) = exprs.find_expr_with_node(node) else {
            return Ok(None);
        };

        let ty = type_of(&exprs, &mut types, root_expr);

        let mut markdown = String::new();
        //markdown += &format!("```lisp\n{:#}\n```", node);

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
        let file_uri = &params.text_document.uri;
        let Ok(file_path) = file_uri.to_file_path() else {
            self.client
                .log_message(MessageType::INFO, &format!("Weird file uri: {file_uri:#?}"))
                .await;
            return Ok(None);
        };
        let file = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|_e| Error::new(ErrorCode::InternalError))?;

        let tree = get_tree(&file);
        let (root_expr, exprs) = from_tree(&tree, &file);
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
                    label: InlayHintLabel::String(format!("{}", ty)),
                    kind: None,
                    text_edits: None,
                    data: None,
                    padding_left: Some(true),
                    padding_right: None,
                    tooltip: None,
                });
            }
        }

        if hints.is_empty() {
            self.client
                .log_message(MessageType::INFO, &format!("No hints: {:#?}", types))
                .await;
            return Ok(None);
        }
        self.client
            .log_message(MessageType::INFO, &format!("Hints: {hints:#?}"))
            .await;

        Ok(Some(hints))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        self.client
            .log_message(MessageType::INFO, &format!("Goto: {params:?}"))
            .await;
        // let location = Location { uri, range };
        // Ok(Some(GotoDefinitionResponse::Scalar(location)))
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
