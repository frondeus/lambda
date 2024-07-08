use std::io::BufWriter;

use ariadne::{Color, Label, Report, Source};
use tree_sitter::Range;

#[derive(Debug, Default)]
pub struct Diagnostics {
    pub errors: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn push(&mut self, node: &Option<crate::ast::SyntaxNode>, error: impl ToString) {
        let span = node.map(|n| n.range()).unwrap_or_else(|| Range {
            start_byte: Default::default(),
            end_byte: Default::default(),
            start_point: Default::default(),
            end_point: Default::default(),
        });
        let diag = Diagnostic {
            span,
            message: error.to_string(),
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
    pub span: tree_sitter::Range,
    pub message: String,
}

impl Diagnostic {
    pub fn to_report(&self) -> Report<'_, (&str, std::ops::Range<usize>)> {
        Report::build(ariadne::ReportKind::Error, "test", self.span.start_byte)
            .with_message(&self.message)
            .with_label(
                Label::new(("test", self.span.start_byte..self.span.end_byte))
                    .with_color(Color::Red),
            )
            .finish()
    }
}

impl Diagnostics {
    pub fn to_pretty_string(&self, input: &str) -> String {
        let mut output = Vec::new();
        let mut output_buf = BufWriter::new(&mut output);
        for diag in self.iter().map(|d| d.to_report()) {
            diag.write(("test", Source::from(input.to_string())), &mut output_buf)
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
            let (r, exprs) = from_tree(&tree, input);
            let mut diagnostics = Diagnostics::default();
            let ir = Exprs::from_ast(&exprs, r, &mut diagnostics);
            _ = TypeEnv::infer(&ir, r, &mut diagnostics);
            diagnostics.to_pretty_string(input)
        })
    }
}
