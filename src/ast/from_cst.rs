use super::builder::*;
use super::ExprId;
use super::Exprs;

pub use tree_sitter::Node as SyntaxNode;
pub use tree_sitter::Tree as SyntaxTree;

pub fn get_tree(code: &str) -> SyntaxTree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_lambda::language())
        .unwrap();

    let tree = parser.parse(code, None).unwrap();
    tree
}

pub fn from_tree(tree: SyntaxTree, code: &str) -> (ExprId, Exprs) {
    let root = tree.root_node();

    let mut cursor = root.walk();
    let root = root
        .children(&mut cursor)
        .filter(|c| !c.is_extra())
        .next()
        .unwrap();

    from_node(root, code).root()
}

pub fn from_source(code: &str) -> (ExprId, Exprs) {
    let tree = get_tree(code);
    from_tree(tree, code)
}

fn from_field<'t>(node: SyntaxNode<'t>, source: &'t str, field: &str) -> impl BuilderFn + 't {
    from_node(node.child_by_field_name(field).unwrap(), source)
}

fn from_node<'t>(node: SyntaxNode<'t>, source: &'t str) -> impl BuilderFn + 't {
    move |e: &mut Exprs| match node.kind() {
        "(" => from_node(node.next_sibling().unwrap(), source).build(e),
        "bool" => match node.child(0).unwrap().kind() {
            "true" => true.build(e),
            "false" => false.build(e),
            kind => todo!("{kind}"),
        }
        "let" => _let(
            from_field_str(node, source, "key"),
            from_field(node, source, "value"),
            from_field(node, source, "in"),
        )
        .build(e),
        "def" => def(
            from_field_str(node, source, "arg"),
            from_field(node, source, "body"),
        )
        .build(e),
        "ident" => var(from_str(node, source)).build(e),
        "call" => call(
            from_field(node, source, "func"),
            from_field(node, source, "arg"),
        )
        .build(e),
        kind => todo!("{kind}"),
    }
}

fn from_field_str<'t>(node: SyntaxNode<'t>, source: &'t str, field: &str) -> &'t str {
    from_str(
        node.child_by_field_name(field)
            .unwrap_or_else(|| panic!("Could not find {field}")),
        source,
    )
}

fn from_str<'t>(node: SyntaxNode<'t>, source: &'t str) -> &'t str {
    node.utf8_text(source.as_bytes()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::ast::builder::*;
    use test_case::test_case;

    use super::*;

    macro_rules! assert_expected {
        ($e: expr, $a: expr) => {{
            let mut exprs = Exprs::default();
            let r = $e.dependency(&mut exprs);
            let expected = exprs.debug(r);
            assert_eq!($a, expected);
        }};
    }

    #[test_case("true", true)]
    #[test_case("false", false)]
    #[test_case("let x = true; false", _let("x", true, false))]
    #[test_case("a: a", def("a", "a"))]
    #[test_case("a b", "a".call("b"))]
    #[test_case("a b c", "a".call_n(("b", "c")))]
    fn test_cst(source: &str, expected: impl BuilderFn) {
        let (r, exprs) = from_source(source);
        let actual = exprs.debug(r);

        assert_expected!(expected, actual);
    }
}
