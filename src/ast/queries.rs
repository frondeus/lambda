use tree_sitter::Node as SyntaxNode;

use super::{Expr, ExprId, Exprs};

impl<'a> Expr<'a> {
    pub fn node(&self) -> Option<SyntaxNode<'a>> {
        match self {
            Expr::Bool { node, .. } => *node,
            Expr::Var { node, .. } => *node,
            Expr::Def { node, .. } => *node,
            Expr::Call { node, .. } => *node,
            Expr::Let { node, .. } => *node,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Expr::Bool { .. } => true,
            Expr::Var { .. } => false,
            Expr::Def { .. } => false,
            Expr::Call { .. } => false,
            Expr::Let { .. } => false,
        }
    }
}

pub trait Queries<'a> {
    fn find_expr_with_node(&self, node: SyntaxNode<'a>) -> Option<ExprId>;
}

impl<'a> Queries<'a> for Exprs<'a> {
    fn find_expr_with_node(&self, node: SyntaxNode<'a>) -> Option<ExprId> {
        for (idx, e) in self.e.iter().enumerate() {
            if let Some(e_node) = e.node() {
                if e_node == node {
                    // if let Some(_) = e_node.child_containing_descendant(node) {
                    return Some(ExprId(idx));
                    // }
                }
            }
        }
        None
    }
}
