use lambda::ast::from_cst::get_tree;
use tower_lsp::{
    jsonrpc::{Error, ErrorCode, Result},
    lsp_types::{
        GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
        HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams,
        MarkedString, MessageType, OneOf, Position, ServerCapabilities,
    },
    Client, LanguageServer,
};

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

        let byte = position_to_byte(&params.text_document_position_params.position, &file);

        let tree = get_tree(&file);
        let root = tree.root_node();
        let node = root.named_descendant_for_byte_range(byte,byte).unwrap();
        let markdown = format!("```lisp\n{:#}\n```", node);

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::from_markdown(markdown.into())),
            range: None,
        }))
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

fn position_to_byte(pos: &Position, code: &str) -> usize{
    let Position  { line, character } = pos;

    let lines = code.lines().collect::<Vec<_>>();
    let line_start = lines.iter().take(*line as usize)
        .map(|l| l.len() + 1).sum::<usize>();
    let byte_position = line_start + *character as usize;
    byte_position
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abc", Position { line: 0, character: 0 } => 0)]
    #[test_case("abc", Position { line: 0, character: 1 } => 1)]
    #[test_case("abc\n\ncde", Position { line: 2, character: 1 } => 6)]
    fn position_to_byte_test(input: &str, pos: Position) -> usize {
        position_to_byte(&pos, input)
    }
}
