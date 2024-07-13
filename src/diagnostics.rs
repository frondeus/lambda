use std::{collections::HashMap, io::BufWriter, sync::Arc};

use ariadne::{sources, Color, Label, Report};
use tree_sitter::Range;

use crate::source::Spanned;

#[derive(Debug, Default)]
pub struct Diagnostics {
    pub errors: Vec<Diagnostic>,
    // pub sources: Vec<(Arc<str>, Arc<str>)>,
    pub sources: HashMap<Arc<str>, Arc<str>>,
}

fn default_range() -> Range {
    Range {
        start_byte: Default::default(),
        end_byte: Default::default(),
        start_point: Default::default(),
        end_point: Default::default(),
    }
}

fn default_source() -> Arc<str> {
    Arc::from("<unknown>")
}

fn default_filename() -> Arc<str> {
    Arc::from("<unknown>")
}

impl Diagnostics {
    pub fn push(&mut self, node: &Option<crate::ast::SyntaxNode>, error: impl ToString) {
        let range = node.as_ref().map(|n| n.range).unwrap_or_else(default_range);
        let source = node
            .as_ref()
            .map(|n| n.source.clone())
            .unwrap_or_else(default_source);
        let filename = node
            .as_ref()
            .map(|n| n.filename.clone())
            .unwrap_or_else(default_filename);

        self.sources
            .entry(filename.clone())
            .or_insert(source.clone());

        let diag = Diagnostic {
            message: Spanned {
                range,
                source,
                filename,
                node: error.to_string(),
            },
        };
        self.errors.push(diag);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.errors.iter()
    }
}

#[derive(Debug)]
pub struct Diagnostic {
    pub message: Spanned<String>, // pub span: tree_sitter::Range,
                                  // pub message: String,
}

impl Diagnostic {
    pub fn to_report(&self) -> Report<'_, (Arc<str>, std::ops::Range<usize>)> {
        let range = self.message.range;
        Report::build(
            ariadne::ReportKind::Error,
            self.message.filename.clone(),
            range.start_byte,
        )
        .with_message(&self.message.node)
        .with_label(
            Label::new((
                self.message.filename.clone(),
                range.start_byte..range.end_byte,
            ))
            .with_color(Color::Red),
        )
        .finish()
    }
}

impl Diagnostics {
    pub fn to_pretty_string(&self) -> String {
        let mut output = Vec::new();
        let mut output_buf = BufWriter::new(&mut output);
        for diag in self.iter().map(|d| d.to_report()) {
            diag.write(sources(self.sources.clone()), &mut output_buf)
                .unwrap();
        }
        drop(output_buf);
        String::from_utf8(output).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::from_cst::{from_tree, get_tree},
        ir::Exprs,
        types::TypeEnv,
    };

    use super::*;

    #[test]
    fn types_tests() -> test_runner::Result {
        test_runner::test_snapshots("tests/", "diagnostics", |input, _deps| {
            let tree = get_tree(input);
            let (r, exprs) = from_tree(&tree, input, "test");
            let mut diagnostics = Diagnostics::default();
            let ir = Exprs::from_ast(&exprs, r, &mut diagnostics);
            _ = TypeEnv::infer(&ir, r, &mut diagnostics);
            diagnostics.to_pretty_string()
        })
    }
}
