use super::builder::*;
use super::ExprId;
use super::Exprs;
use super::{SyntaxNode, SyntaxTree};

pub fn get_tree(code: &str) -> SyntaxTree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_lambda::language())
        .unwrap();

    parser.parse(code, None).unwrap()
}

pub fn get_tree_diff(code: &str, old: &SyntaxTree) -> SyntaxTree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_lambda::language())
        .unwrap();

    parser.parse(code, Some(old)).unwrap()
}

pub fn from_tree<'t>(tree: &'t SyntaxTree, code: &'t str) -> (ExprId, Exprs<'t>) {
    let root = tree.root_node();

    let mut cursor = root.walk();
    let root = root.children(&mut cursor).find(|c| !c.is_extra()).unwrap();

    from_node(root, code).root()
}

// pub fn from_source(code: &str) -> (ExprId, Exprs) {
//     let tree = get_tree(code);
//     from_tree(&tree, code)
// }

fn from_field<'t>(node: SyntaxNode<'t>, source: &'t str, field: &str) -> impl BuilderFn<'t> + 't {
    from_node(node.child_by_field_name(field).unwrap(), source)
}

fn from_node<'t>(node: SyntaxNode<'t>, source: &'t str) -> impl BuilderFn<'t> + 't {
    move |e: &mut Exprs<'t>| match node.kind() {
        "(" => from_node(node.next_sibling().unwrap(), source).build(e),
        "bool" => match node.child(0).unwrap().kind() {
            "true" => true.build_with_node(e, node),
            "false" => false.build_with_node(e, node),
            kind => todo!("{kind}"),
        },
        "let" => _let(
            from_var_def(node, source, "key"),
            from_field(node, source, "value"),
            from_field(node, source, "in"),
        )
        .build_with_node(e, node),
        "def" => def(
            from_var_def(node, source, "arg"),
            from_field(node, source, "body"),
        )
        .build_with_node(e, node),
        "ident" => var(from_str(node, source)).build_with_node(e, node),
        "call" => call(
            from_field(node, source, "func"),
            from_field(node, source, "arg"),
        )
        .build_with_node(e, node),
        kind => todo!("{kind}"),
    }
}

fn from_var_def<'t>(node: SyntaxNode<'t>, source: &'t str, field: &str) -> impl VarDefLike<'t> {
    let node = node.child_by_field_name(field).unwrap();
    VarDef {
        arg: from_str(node, source),
        node: Some(node),
    }
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
        call(
            _let("g", def("a", "a"), "g"),
            true
        ) ;
         "Whitespace 2")
    ]
    #[test_case("a: a", def("a", "a"))]
    #[test_case("a b", "a".call("b"))]
    #[test_case("a b c", "a".call_n(("b", "c")))]
    fn test_cst<'t>(source: &'t str, expected: impl BuilderFn<'t>) {
        let tree = get_tree(source);
        let (r, exprs) = from_tree(&tree, source);
        let actual = exprs.debug(r);

        assert_expected!(expected, actual);
    }
}
