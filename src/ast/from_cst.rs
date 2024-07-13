use std::sync::Arc;

use crate::source::Spanned;

use super::builder::*;
use super::ExprId;
use super::Exprs;
use super::{SyntaxNode, SyntaxTree};

#[allow(clippy::expect_used)]
pub fn get_tree(code: &str) -> SyntaxTree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_lambda::language())
        .expect("Parser");

    parser.parse(code, None).expect("Tree")
}

#[allow(clippy::expect_used)]
pub fn get_tree_diff(code: &str, old: &SyntaxTree) -> SyntaxTree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_lambda::language())
        .expect("Parser");

    parser.parse(code, Some(old)).expect("Tree")
}

pub fn to_spanned<'t>(
    node: tree_sitter::Node<'t>,
    source: &'t str,
) -> Spanned<tree_sitter::Node<'t>> {
    Spanned {
        range: node.range(),
        filename: Arc::from("<test>"),
        source: Arc::from(source),
        node,
    }
}

pub fn from_tree<'t>(
    tree: &'t SyntaxTree,
    code: &'t str,
    filename: &'t str,
) -> (Option<ExprId>, Exprs<'t>) {
    let root = tree.root_node();

    let mut cursor = root.walk();
    let root = root.children(&mut cursor).find(|c| !c.is_extra());
    let root = root.map(|root| Spanned {
        range: root.range(),
        filename: Arc::from(filename),
        source: Arc::from(code),
        node: root,
    });

    from_maybe_node(root).root()
}

// pub fn from_source(code: &str) -> (ExprId, Exprs) {
//     let tree = get_tree(code);
//     from_tree(&tree, code)
// }

fn from_field<'t>(node: SyntaxNode<'t>, field: &str) -> impl BuilderFn<'t> + 't {
    from_maybe_node(node.map(|t| t.child_by_field_name(field)).transpose())
}

fn from_maybe_node<'t>(node: Option<SyntaxNode<'t>>) -> impl BuilderFn<'t> + 't {
    move |e: &mut Exprs<'t>| match node {
        Some(node) => from_node(node).build(e),
        None => None,
    }
}

fn from_node<'t>(node: SyntaxNode<'t>) -> impl BuilderFn<'t> + 't {
    move |e: &mut Exprs<'t>| match node.node.kind() {
        "(" => from_maybe_node(node.map(|node| node.next_sibling()).transpose()).build(e),
        "bool" => match node.node.child(0).map(|n| n.kind()) {
            Some("true") => true.build_with_node(e, node),
            Some("false") => false.build_with_node(e, node),
            kind => todo!("{kind:?}"),
        },
        "let" => _let(
            from_var_def(node.clone(), "key"),
            from_field(node.clone(), "value"),
            from_field(node.clone(), "in"),
        )
        .build_with_node(e, node),
        "def" => def(
            from_var_def(node.clone(), "arg"),
            from_field(node.clone(), "body"),
        )
        .build_with_node(e, node),
        "ident" => var(from_str(node.clone())).build_with_node(e, node),
        "call" => call(
            from_field(node.clone(), "func"),
            from_field(node.clone(), "arg"),
        )
        .build_with_node(e, node),
        kind => todo!("{kind}"),
    }
}

fn from_var_def<'t>(node: SyntaxNode<'t>, field: &str) -> impl VarDefLike<'t> {
    let node = node.map(|node| node.child_by_field_name(field)).transpose();
    VarDef {
        arg: node.clone().map(from_str).unwrap_or_default(),
        node,
    }
}

#[allow(clippy::expect_used)]
/// PANICS: Our lambda language always expects utf8
fn from_str(node: SyntaxNode) -> String {
    node.node
        .utf8_text(node.source.as_bytes())
        .expect("UTF8 text")
        .into()
}

#[cfg(test)]
mod tests {
    use crate::ast::builder::*;
    use test_case::test_case;

    use super::*;

    #[test]
    fn cst_tests() -> test_runner::Result {
        test_runner::test_snapshots("tests/", "cst", |input, _deps| {
            let tree = get_tree(input);
            format!("{:#}", tree.root_node())
        })
    }

    #[test]
    fn ast_tests() -> test_runner::Result {
        test_runner::test_snapshots("tests/", "ast", |input, _deps| {
            let tree = get_tree(input);
            let (r, exprs) = from_tree(&tree, input, "test");
            format!("{:#?}", exprs.debug(r))
        })
    }

    macro_rules! assert_expected {
        ($e: expr, $a: expr) => {{
            let mut exprs = Exprs::default();
            let r = $e.dependency(&mut exprs);
            let expected = exprs.debug(r);
            let e = format!("{:?}", expected);
            let a = format!("{:?}", $a);
            assert_eq!(e, a);
        }};
    }

    #[test_case("true", true)]
    #[test_case("false", false)]
    #[test_case("let x = true; false", _let("x", true, false))]
    #[test_case("let x = true;     \n\n\n false", _let("x", true, false) ; "With whitespace")]
    #[test_case("let g = a: a;         \ng\ntrue\n", 
            _let("g", def("a", "a"), "g".call(true));
         "Whitespace 2")
    ]
    #[test_case("a: a", def("a", "a"))]
    #[test_case("a b", "a".call("b"))]
    #[test_case("a b c", "a".call_n(("b", "c")))]
    fn test_cst<'t>(source: &'t str, expected: impl BuilderFn<'t>) {
        let tree = get_tree(source);
        let (r, exprs) = from_tree(&tree, source, "test");
        let actual = exprs.debug(r);

        assert_expected!(expected, actual);
    }
}
