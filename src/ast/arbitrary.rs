use arbitrary::{Arbitrary, Result, Unstructured};

use super::{
    builder::{var, BuilderFn},
    Expr, ExprId, Exprs,
};

const NAMES: &[&str] = &["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"];

pub fn arbitrary_expr_id(e: &mut Exprs, u: &mut Unstructured) -> Result<ExprId> {
    let expr = arbitrary_expr(e, u)?;
    Ok(e.push(expr))
}

fn arbitrary_expr<'a>(e: &mut Exprs<'a>, u: &mut Unstructured) -> Result<Expr<'a>> {
    let kind = u.arbitrary::<ExprKind>()?;
    Ok(match kind {
        ExprKind::Bool => Expr::Bool {
            value: u.arbitrary()?,
            node: None,
        },
        ExprKind::Var => var(u.choose(NAMES)?).build(e),
        ExprKind::Def => {
            let ret = arbitrary_expr_id(e, u)?;
            let name = e.push_str(u.choose(NAMES)?);
            let name = e.push(Expr::VarDef { name, node: None });
            Expr::Def {
                arg: name,
                body: ret,
                node: None,
            }
        }
        ExprKind::Call => {
            let func = arbitrary_expr_id(e, u)?;
            let arg = arbitrary_expr_id(e, u)?;
            Expr::Call {
                func,
                arg,
                node: None,
            }
        }
        ExprKind::Let => {
            let name = e.push_str(u.choose(NAMES)?);
            let name = e.push(Expr::VarDef { name, node: None });
            let value = arbitrary_expr_id(e, u)?;
            let then = arbitrary_expr_id(e, u)?;
            Expr::Let {
                name,
                value,
                body: then,
                node: None,
            }
        }
    })
}

#[derive(Arbitrary)]
enum ExprKind {
    Bool,
    Var,
    Def,
    Call,
    Let,
}

#[cfg(test)]
mod tests {
    use crate::{diagnostics::Diagnostics, runtime::RunEnv, types::TypeEnv};

    use super::*;

    #[test]
    fn fuzzy_tests() {
        arbtest::arbtest(|u| {
            let mut exprs = Exprs::default();
            let root = arbitrary_expr_id(&mut exprs, u)?;
            let mut rt = RunEnv::default();
            let mut diagnostics = Diagnostics::default();
            let ir = crate::ir::Exprs::from_ast(&exprs, root, &mut diagnostics);

            _ = TypeEnv::infer(&ir, root, &mut diagnostics);
            if diagnostics.has_errors() {
                return Ok(());
            }
            crate::runtime::eval(&exprs, &mut rt, root);
            Ok(())
        });
        // .budget_ms(5_000);
    }
}
