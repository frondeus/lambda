use crate::ast::{Expr, ExprId, Exprs};

use super::{Con, TermId, TypeEnv};

impl std::fmt::Debug for TermId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$t{}", self.0)
    }
}
impl std::fmt::Debug for TypeEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (e_id, e) in self.exprs.iter() {
            writeln!(f, "| {e_id:?} | {e:?} |")?;
        }
        writeln!(f, "---")?;
        for (t_id, t) in self.terms.iter().enumerate() {
            writeln!(f, "| {:?} | {t:?} |", TermId(t_id))?;
        }
        Ok(())
    }
}
impl std::fmt::Debug for Con {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} = {:?}", self.left, self.right)
    }
}

pub fn expr_print(exprs: &Exprs, id: ExprId, env: &TypeEnv) {
    fn expr_print_inner(
        exprs: &Exprs,
        id: ExprId,
        env: &TypeEnv,
        indent: usize,
        ignore_indent: bool,
    ) {
        let term_id = env.term_id_of(id).expect("Term");
        let term = env.print_term_id(term_id);
        let print_indent = |indent, ignore_indent: bool| {
            if !ignore_indent {
                print!("{ }", " ".repeat(indent * 4));
            }
        };
        print_indent(indent, ignore_indent);
        match exprs.get(id) {
            Expr::Bool(true) => println!("true # {term} ({term_id:?})"),
            Expr::Bool(false) => println!("false # {term} ({term_id:?})"),
            Expr::Var(name) => println!("{name} # {term} ({term_id:?})"),
            Expr::Def(arg, ret) => {
                println!("fn ({arg}) => # {term} ({term_id:?})");
                expr_print_inner(exprs, *ret, env, indent + 1, false);
            }
            Expr::Call(f, a) => {
                expr_print_inner(exprs, *f, env, indent, true);
                print_indent(indent, false);
                println!("(");
                expr_print_inner(exprs, *a, env, indent + 1, false);
                print_indent(indent, false);
                println!(") # {term} ({term_id:?})");
            }
            Expr::Let(name, value, then) => {
                print!("let {name} = ");
                expr_print_inner(exprs, *value, env, indent, true);
                print_indent(indent, ignore_indent);
                println!("in");
                expr_print_inner(exprs, *then, env, indent + 1, false);
            }
        }
    }

    expr_print_inner(exprs, id, env, 0, false);
}
