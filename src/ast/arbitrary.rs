use arbitrary::{Arbitrary, Result, Unstructured};

use super::{Expr, ExprId, Exprs};

const NAMES: &[&str] = &["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"];

pub fn arbitrary_expr_id(e: &mut Exprs, u: &mut Unstructured) -> Result<ExprId> {
    let expr = arbitrary_expr(e, u)?;
    Ok(e.push(expr))
}

fn arbitrary_expr(e: &mut Exprs, u: &mut Unstructured) -> Result<Expr> {
    let kind = u.arbitrary::<ExprKind>()?;
    Ok(match kind {
        ExprKind::Bool => Expr::Bool(u.arbitrary()?),
        ExprKind::Var => Expr::Var(u.choose(NAMES)?),
        ExprKind::Def => {
            let ret = arbitrary_expr_id(e, u)?;
            Expr::Def(u.choose(NAMES)?, ret)
        }
        ExprKind::Call => {
            let func = arbitrary_expr_id(e, u)?;
            let arg = arbitrary_expr_id(e, u)?;
            Expr::Call(func, arg)
        }
        ExprKind::Let => {
            let name = u.choose(NAMES)?;
            let value = arbitrary_expr_id(e, u)?;
            let then = arbitrary_expr_id(e, u)?;
            Expr::Let(name, value, then)
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
    use crate::{runtime::RunEnv, types::TypeEnv};

    use super::*;

    #[test]
    fn fuzzy_tests() {
        arbtest::arbtest(|u| {
            let mut exprs = Exprs::default();
            let root = arbitrary_expr_id(&mut exprs, u)?;
            let mut types = TypeEnv::default();
            let mut rt = RunEnv::default();

            let term = crate::types::type_of(&exprs, &mut types, root);
            if term.is_ok() {
                crate::runtime::eval(&exprs, &mut rt, root);
            }
            Ok(())
        })
        // .budget_ms(5_000)
        ;
    }
}
