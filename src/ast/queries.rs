use super::{Expr, ExprId, Exprs, SyntaxNode};

impl<'a> Expr<'a> {
    pub fn node(&self) -> Option<SyntaxNode<'a>> {
        match self {
            Expr::Bool { node, .. } => node.clone(),
            Expr::Var { node, .. } => node.clone(),
            Expr::VarDef { node, .. } => node.clone(),
            Expr::Def { node, .. } => node.clone(),
            Expr::Call { node, .. } => node.clone(),
            Expr::IfElse { node, .. } => node.clone(),
            Expr::Let { node, .. } => node.clone(),
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Expr::Bool { .. } => true,
            Expr::Var { .. } => false,
            Expr::VarDef { .. } => false,
            Expr::Def { .. } => false,
            Expr::Call { .. } => false,
            Expr::IfElse { .. } => false,
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
                // tracing::info!("* {idx} - {e_node} - {e:?}");
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
